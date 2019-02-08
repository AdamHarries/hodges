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

  // small optimisation to make it faster to print to stdout

  static float buffer[buffer_size];
  int ix = 0;

  void* state = init_state(argv[1]);

  enum YieldState status;

  status = advance_float_iterator(state);
  while (status == DataAvailable) {
    buffer[ix++] = get_float(state);
    if (ix == buffer_size) {
      fwrite(buffer, sizeof(float), buffer_size, stdout);
      ix = 0;
    }
    status = advance_float_iterator(state);
  }
  if (ix != 0) {
    fwrite(buffer, sizeof(float), ix, stdout);
  }

  cleanup(state);
}