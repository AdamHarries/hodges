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

  // Read frame loop state
  enum YieldState read_frame;

  // Recieve frame loop state
  enum YieldState recieve_frame;

  // Get frame loop state
  enum YieldState get_frame;

  //   enum SourceState rf_state;

  // char decoding state
  //   enum SourceState cs_state;
  enum YieldState char_yield_status;
  int samples;
  output_t* arr_ix;
  output_t* arr_end;
  int i;

  // The latest value.
  char v;
} PgState;
