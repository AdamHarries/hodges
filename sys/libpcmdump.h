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

typedef float output_t;

enum YieldState {
  DataAvailable,
  Finished,
  FinishedWithError,
  DecoderSendError,
  FrameRecieveError,
  FiltergraphFeedError,
  FiltergraphPullError
};

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
} PgState;

static int open_input_file(const char* filename, PgState* state);

static int init_filters(const char* filters_descr, PgState* state);

void cleanup(PgState* state);

PgState* init_state(const char* filename);

enum YieldState pcmdump_log_err(enum YieldState errcode);

/* extern inline */ enum YieldState send_packet(PgState* state);

/* extern inline */ enum YieldState recv_frame(PgState* state);

/* extern inline */ enum YieldState pull_frame(PgState* state);

/* extern inline */ enum YieldState get_sample(PgState* state);

#endif