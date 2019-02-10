#ifndef LIBPCM_DUMP_H
#define LIBPCM_DUMP_H

// Output format:
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

void cleanup(void* st);

void* init_state(const char* filename);

enum YieldState advance_char_iterator(void* st);

char get_char(void* st);

enum YieldState advance_float_iterator(void* st);
float get_float(void* st);

enum YieldState get_char_buffer(void* st, char** buffer, int* samples);

enum YieldState get_float_buffer(void* st, float** buffer, int* samples);
#endif