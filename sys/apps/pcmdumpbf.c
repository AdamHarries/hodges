#include <stdio.h>
#include <stdlib.h>
#include "libhodges.h"

#define buffer_size 65536
int main(int argc, char** argv) {
  if (argc != 2) {
    static const char* player = "ffplay -f f32le -ar 44100 -ac 1 -";

    fprintf(stderr, "Usage: %s file | %s\n", argv[0], player);
    exit(1);
  }

  void* state = init_state(argv[1]);
  
  float* buffer;
  int samples;

  enum YieldState status = get_float_buffer(state, &buffer, &samples);
  while (status == DataAvailable) {
    fwrite(buffer, sizeof(float), samples, stdout);
    status = get_float_buffer(state, &buffer, &samples);
  }

  cleanup(state);
}