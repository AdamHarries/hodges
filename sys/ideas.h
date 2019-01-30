#include "libpcmdump.h"

enum YieldState send_packet(PgState* state) {
  // Layer One
  av_packet_unref(&(state->packet));
  if ((state->ret = av_read_frame(state->fmt_ctx, &(state->packet))) < 0) {
    return Finished;
  }

  if (state->packet.stream_index == state->audio_stream_index) {
    // Send data to the layer below
    state->ret = avcodec_send_packet(state->dec_ctx, &(state->packet));

    if (state->ret < 0) {
      av_log(NULL, AV_LOG_ERROR,
             "Error while sending a packet to the decoder\n");
      return FinishedWithError;
    }
  }
  return DataAvailable;
}

enum YieldState recv_frame(PgState* state) {
  enum YieldState status;
  av_frame_unref(state->frame);
  state->ret = avcodec_receive_frame(state->dec_ctx, state->frame);
  while (state->ret == AVERROR(EAGAIN) || state->ret == AVERROR_EOF) {
    status = send_packet(state);
    if (status == FinishedWithError || status == Finished) {
      return status;
    }
    state->ret = avcodec_receive_frame(state->dec_ctx, state->frame);
  }

  if (state->ret < 0) {
    av_log(NULL, AV_LOG_ERROR,
           "Error while receiving a frame from the decoder\n");
    return FinishedWithError;
  }

  /* push the audio data from decoded frame into the filtergraph */
  if (av_buffersrc_add_frame_flags(state->buffersrc_ctx, state->frame,
                                   AV_BUFFERSRC_FLAG_KEEP_REF) < 0) {
    av_log(NULL, AV_LOG_ERROR, "Error while feeding the audio filtergraph\n");
    return FinishedWithError;
  }

  return DataAvailable;
}

enum YieldState pull_frame(PgState* state) {
  enum YieldState status;
  av_frame_unref(state->filt_frame);
  /* pull filtered audio from the filtergraph */
  state->ret =
      av_buffersink_get_frame(state->buffersink_ctx, state->filt_frame);
  while (state->ret == AVERROR(EAGAIN) || state->ret == AVERROR_EOF) {
    status = recv_frame(state);
    if (status == Finished || status == FinishedWithError) {
      return status;
    }
    state->ret =
        av_buffersink_get_frame(state->buffersink_ctx, state->filt_frame);
  }
  if (state->ret < 0) {
    av_log(NULL, AV_LOG_ERROR, "Error while pulling frame from filtergraph\n");
    return FinishedWithError;
  }

  state->samples =
      state->filt_frame->nb_samples *
      av_get_channel_layout_nb_channels(state->filt_frame->channel_layout);

  state->arr_ix = (output_t*)state->filt_frame->data[0];
  state->arr_end = state->arr_ix + state->samples;
  state->i = 0;
  return DataAvailable;
}

enum YieldState get_sample(PgState* state) {
  enum YieldState status;

  if (state->should_send_packet) {
    status = send_packet(state);
    if (status == Finished || status == FinishedWithError) {
      return status;
    }
    state->should_send_packet = 0;
  }

recv_frame:
  if (state->should_recv_frame) {
    status = recv_frame(state);
    if (status == Finished || status == FinishedWithError) {
      return status;
    }
    state->should_recv_frame = 0;
  }

pull_frame:
  if (state->should_pull_frame) {
    pull_frame(state);
    if (status == Finished || status == FinishedWithError) {
      return status;
    }
    state->should_pull_frame = 0;
  }

  // Actually print a frame!
  char r;

yield_char:

  r = *((char*)state->arr_ix + (state->i));

  state->i++;

  if (!(state->i < sizeof(output_t))) {
    state->i = 0;
    state->arr_ix++;
  }

  fputc(r, stdout);

  if (state->arr_ix < state->arr_end) {
    goto yield_char;
  }

  fflush(stdout);

  av_frame_unref(state->filt_frame);
  av_frame_unref(state->frame);
  goto recv_frame;
}