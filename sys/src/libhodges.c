
/**
 * @file
 * API example for audio decoding and filtering
 * @example filtering_audio.c
 */
#include <unistd.h>

#include <libavcodec/avcodec.h>
// #include <libavfilter/avfiltergraph.h>
#include <libavfilter/buffersink.h>
#include <libavfilter/buffersrc.h>
#include <libavformat/avformat.h>
#include <libavutil/opt.h>

#include "libhodges.h"

typedef struct {
  // A generic return value for use by functions
  int ret;
  // Overall status
  AVFormatContext* fmt_ctx;
  AVCodecContext* dec_ctx;
  AVFilterContext* buffersink_ctx;
  AVFilterContext* buffersrc_ctx;
  AVFilterGraph* filter_graph;
  int audio_stream_index;

  // decoding state
  AVPacket packet;
  AVFrame* frame;
  AVFrame* filt_frame;
  // char decoding state
  // Note, these *must* be zero initialised (either with calloc, or manually)
  int samples;
  output_t* arr_ix;
  output_t* arr_end;
  int i;

  // The latest value.
  char v;
  float f;
} PgState;

#define TRY_CALL(fcall)                   \
  enum YieldState try_call_status;        \
  try_call_status = fcall;                \
  if (try_call_status != DataAvailable) { \
    return try_call_status;               \
  }

#define CHECK_RETURN(err_code)        \
  if (state->ret < 0) {               \
    return pcmdump_log_err(err_code); \
  }

static int open_input_file(const char* filename, PgState* state) {
  int ret;
  AVCodec* dec;

  if ((ret = avformat_open_input(&state->fmt_ctx, filename, NULL, NULL)) < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot open input file\n");
    return ret;
  }

  if ((ret = avformat_find_stream_info(state->fmt_ctx, NULL)) < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot find stream information\n");
    return ret;
  }

  /* select the audio stream */
  ret =
      av_find_best_stream(state->fmt_ctx, AVMEDIA_TYPE_AUDIO, -1, -1, &dec, 0);
  if (ret < 0) {
    av_log(NULL, AV_LOG_ERROR,
           "Cannot find an audio stream in the input file\n");
    return ret;
  }
  state->audio_stream_index = ret;

  /* create decoding context */
  state->dec_ctx = avcodec_alloc_context3(dec);
  if (!state->dec_ctx)
    return AVERROR(ENOMEM);
  avcodec_parameters_to_context(
      state->dec_ctx,
      state->fmt_ctx->streams[state->audio_stream_index]->codecpar);
  av_opt_set_int(state->dec_ctx, "refcounted_frames", 1, 0);

  /* init the audio decoder */
  if ((ret = avcodec_open2(state->dec_ctx, dec, NULL)) < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot open audio decoder\n");
    return ret;
  }

  return 0;
}

static int init_filters(const char* filters_descr, PgState* state) {
  char args[512];
  int ret = 0;
  AVFilter* abuffersrc = avfilter_get_by_name("abuffer");
  AVFilter* abuffersink = avfilter_get_by_name("abuffersink");
  AVFilterInOut* outputs = avfilter_inout_alloc();
  AVFilterInOut* inputs = avfilter_inout_alloc();
  static const enum AVSampleFormat out_sample_fmts[] = {AV_SAMPLE_FMT_FLT, -1};
  static const int64_t out_channel_layouts[] = {AV_CH_LAYOUT_MONO, -1};
  static const int out_sample_rates[] = {44100, -1};
  const AVFilterLink* outlink;
  AVRational time_base =
      state->fmt_ctx->streams[state->audio_stream_index]->time_base;

  state->filter_graph = avfilter_graph_alloc();
  if (!outputs || !inputs || !state->filter_graph) {
    ret = AVERROR(ENOMEM);
    goto end;
  }

  /* buffer audio source: the decoded frames from the decoder will be inserted
   * here. */
  if (!state->dec_ctx->channel_layout)
    state->dec_ctx->channel_layout =
        av_get_default_channel_layout(state->dec_ctx->channels);
  snprintf(
      args, sizeof(args),
      "time_base=%d/%d:sample_rate=%d:sample_fmt=%s:channel_layout=0x%" PRIx64,
      time_base.num, time_base.den, state->dec_ctx->sample_rate,
      av_get_sample_fmt_name(state->dec_ctx->sample_fmt),
      state->dec_ctx->channel_layout);
  ret = avfilter_graph_create_filter(&state->buffersrc_ctx, abuffersrc, "in",
                                     args, NULL, state->filter_graph);
  if (ret < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot create audio buffer source\n");
    goto end;
  }

  /* buffer audio sink: to terminate the filter chain. */
  ret = avfilter_graph_create_filter(&state->buffersink_ctx, abuffersink, "out",
                                     NULL, NULL, state->filter_graph);
  if (ret < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot create audio buffer sink\n");
    goto end;
  }

  ret = av_opt_set_int_list(state->buffersink_ctx, "sample_fmts",
                            out_sample_fmts, -1, AV_OPT_SEARCH_CHILDREN);
  if (ret < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot set output sample format\n");
    goto end;
  }

  ret = av_opt_set_int_list(state->buffersink_ctx, "channel_layouts",
                            out_channel_layouts, -1, AV_OPT_SEARCH_CHILDREN);
  if (ret < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot set output channel layout\n");
    goto end;
  }

  ret = av_opt_set_int_list(state->buffersink_ctx, "sample_rates",
                            out_sample_rates, -1, AV_OPT_SEARCH_CHILDREN);
  if (ret < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot set output sample rate\n");
    goto end;
  }

  /*
   * Set the endpoints for the filter graph. The state->filter_graph will
   * be linked to the graph described by filters_descr.
   */

  /*
   * The buffer source output must be connected to the input pad of
   * the first filter described by filters_descr; since the first
   * filter input label is not specified, it is set to "in" by
   * default.
   */
  outputs->name = av_strdup("in");
  outputs->filter_ctx = state->buffersrc_ctx;
  outputs->pad_idx = 0;
  outputs->next = NULL;

  /*
   * The buffer sink input must be connected to the output pad of
   * the last filter described by filters_descr; since the last
   * filter output label is not specified, it is set to "out" by
   * default.
   */
  inputs->name = av_strdup("out");
  inputs->filter_ctx = state->buffersink_ctx;
  inputs->pad_idx = 0;
  inputs->next = NULL;

  if ((ret = avfilter_graph_parse_ptr(state->filter_graph, filters_descr,
                                      &inputs, &outputs, NULL)) < 0)
    goto end;

  if ((ret = avfilter_graph_config(state->filter_graph, NULL)) < 0)
    goto end;

  /* Print summary of the sink buffer
   * Note: args buffer is reused to store channel layout string */
  outlink = state->buffersink_ctx->inputs[0];

end:
  avfilter_inout_free(&inputs);
  avfilter_inout_free(&outputs);

  return ret;
}

static inline enum YieldState pcmdump_log_err(enum YieldState errcode) {
#ifdef DEBUG
  switch (errcode) {
    case DataAvailable:
      av_log(NULL, AV_LOG_ERROR, "Data still available\n");
      break;
    case Finished:
      av_log(NULL, AV_LOG_ERROR, "Pipeline finished successfully\n");
      break;
    case FinishedWithError:
      av_log(NULL, AV_LOG_ERROR, "Pipeline finished with unknown error\n");
      break;
    case DecoderSendError:
      av_log(NULL, AV_LOG_ERROR,
             "Error while sending a packet to the decoder\n");
      break;
    case FrameRecieveError:
      av_log(NULL, AV_LOG_ERROR,
             "Error while receiving a frame from the decoder\n");
      break;
    case FiltergraphFeedError:
      av_log(NULL, AV_LOG_ERROR, "Error while feeding the audio filtergraph\n");
      break;
    case FiltergraphPullError:
      av_log(NULL, AV_LOG_ERROR,
             "Error while pulling frame from filtergraph\n");
      break;
    default:
      av_log(NULL, AV_LOG_ERROR, "Unknown error occurred!");
      break;
  }
#endif
  return errcode;
}

static inline enum YieldState send_packet(PgState* state) {
  // av_packet_unref(&(state->packet));
  if ((state->ret = av_read_frame(state->fmt_ctx, &(state->packet))) < 0) {
    return Finished;
  }

  if (state->packet.stream_index == state->audio_stream_index) {
    // Send data to the layer below
    state->ret = avcodec_send_packet(state->dec_ctx, &(state->packet));

    CHECK_RETURN(DecoderSendError);
  }
  return DataAvailable;
}

static inline enum YieldState recv_frame(PgState* state) {
  // av_frame_unref(state->frame);
  state->ret = avcodec_receive_frame(state->dec_ctx, state->frame);

  while (state->ret == AVERROR(EAGAIN) || state->ret == AVERROR_EOF) {
    TRY_CALL(send_packet(state));
    state->ret = avcodec_receive_frame(state->dec_ctx, state->frame);
  }

  CHECK_RETURN(FrameRecieveError);

  /* push the audio data from decoded frame into the filtergraph */
  state->ret = av_buffersrc_add_frame_flags(state->buffersrc_ctx, state->frame,
                                            AV_BUFFERSRC_FLAG_KEEP_REF);

  CHECK_RETURN(FiltergraphFeedError);

  return DataAvailable;
}

static inline enum YieldState pull_frame(PgState* state) {
  // av_frame_unref(state->filt_frame);
  /* pull filtered audio from the filtergraph */
  state->ret =
      av_buffersink_get_frame(state->buffersink_ctx, state->filt_frame);

  while (state->ret == AVERROR(EAGAIN) || state->ret == AVERROR_EOF) {
    TRY_CALL(recv_frame(state));
    state->ret =
        av_buffersink_get_frame(state->buffersink_ctx, state->filt_frame);
  }

  CHECK_RETURN(FiltergraphPullError);

  state->samples =
      state->filt_frame->nb_samples *
      av_get_channel_layout_nb_channels(state->filt_frame->channel_layout);

  state->arr_ix = (output_t*)state->filt_frame->data[0];
  state->arr_end = state->arr_ix + state->samples;
  state->i = 0;
  return DataAvailable;
}

/* ================
   public interface
   ================ */

void* init_state(const char* filename) {
  int ret;

  av_log_set_level(AV_LOG_FATAL);
  PgState* state = (PgState*)calloc(1, sizeof(PgState));

  state->frame = av_frame_alloc();
  state->filt_frame = av_frame_alloc();

  if (!state->frame || !state->filt_frame) {
    perror("Could not allocate frame");
    return NULL;
  }

  av_register_all();
  avfilter_register_all();

  if ((state->ret = open_input_file(filename, state)) < 0) {
    cleanup(state);
    return NULL;
  }

  static const char* filter_descr =
      "aresample=44100,aformat=sample_fmts=flt:channel_layouts=mono";

  if ((state->ret = init_filters(filter_descr, state)) < 0) {
    cleanup(state);
    return NULL;
  }

  return (void*)state;
}

void cleanup(void* st) {
  PgState* state = (PgState*)st;
  avfilter_graph_free(&state->filter_graph);
  avcodec_free_context(&state->dec_ctx);
  avformat_close_input(&state->fmt_ctx);
  av_frame_free(&state->frame);
  av_frame_free(&state->filt_frame);

  if (state->ret < 0 && state->ret != AVERROR_EOF) {
    fprintf(stderr, "Error occurred: %s\n", av_err2str(state->ret));
  }

  free(state);
}
enum YieldState advance_char_iterator(void* st) {
  PgState* state = (PgState*)st;
  if (!(state->arr_ix < state->arr_end)) {
    TRY_CALL(pull_frame(state));
  }

  state->v = *((char*)state->arr_ix + (state->i));

  state->i++;

  if (!(state->i < sizeof(output_t))) {
    state->i = 0;
    state->arr_ix++;
  }

  return DataAvailable;
}

char get_char(void* st) {
  PgState* state = (PgState*)st;
  return state->v;
}

enum YieldState advance_float_iterator(void* st) {
  PgState* state = (PgState*)st;
  if (!(state->arr_ix < state->arr_end)) {
    TRY_CALL(pull_frame(state));
  }

  state->f = *state->arr_ix;
  state->arr_ix++;

  return DataAvailable;
}

float get_float(void* st) {
  PgState* state = (PgState*)st;
  return state->f;
}

enum YieldState get_char_buffer(void* st, char** buffer, int* samples) {
  PgState* state = (PgState*)st;
  TRY_CALL(pull_frame(state));

  *buffer = (char*)(state->arr_ix);
  *samples = (state->samples) / sizeof(output_t);

  return DataAvailable;
}

enum YieldState get_float_buffer(void* st, float** buffer, int* samples) {
  PgState* state = (PgState*)st;
  TRY_CALL(pull_frame(state));

  *buffer = state->arr_ix;
  *samples = state->samples;

  return DataAvailable;
}