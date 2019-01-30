#ifndef LIBPCM_DUMP_H
#define LIBPCM_DUMP_H

#include <unistd.h>

#include <libavcodec/avcodec.h>
#include <libavfilter/avfiltergraph.h>
#include <libavfilter/buffersink.h>
#include <libavfilter/buffersrc.h>
#include <libavformat/avformat.h>
#include <libavutil/opt.h>

// We want to achieve:
// - format: f32le
// - codec: pcm_f32le
// - channels: 1
// - rate: 44100

// static AVFormatContext* state->fmt_ctx;
// static AVCodecContext* state->dec_ctx;
// AVFilterContext* state->buffersink_ctx;
// AVFilterContext* state->buffersrc_ctx;
// AVFilterGraph* state->filter_graph;
// static int state->audio_stream_index = -1;

typedef float output_t;
enum YieldState { DataAvailable, Finished, FinishedWithError };

// enum SourceState { DataAvailable, NeedMoreData };

enum DataState { NeedsData, Saturated };

enum ErrorState { ErrorReadingSinkFrame };

typedef struct {
  // Overall status
  AVFormatContext* fmt_ctx;
  AVCodecContext* dec_ctx;
  AVFilterContext* buffersink_ctx;
  AVFilterContext* buffersrc_ctx;
  AVFilterGraph* filter_graph;
  int audio_stream_index;

  // A generic return value for use by functions
  int ret;

  // decoding state
  AVPacket packet;
  AVFrame* frame;
  AVFrame* filt_frame;

  char should_send_packet;
  char should_recv_frame;
  char should_pull_frame;

  // does the "recv_frame" section need more data
  enum DataState recv_frame_state;
  // Does the "pull_frame" section need more data?
  enum DataState pull_state;
  // Does the char decoder need more data?
  enum DataState char_state;

  // state for decoding chars
  int samples;
  output_t* arr_ix;
  output_t* arr_end;
  int i;

  // The latest value.
  char v;
} PgState;

#endif