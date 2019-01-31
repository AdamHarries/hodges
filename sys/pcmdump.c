#include "libpcmdump.h"
#include "stdio.h"
int main(int argc, char** argv) {
  if (argc != 2) {
    static const char* player = "ffplay -f f32le -ar 44100 -ac 1 -";

    fprintf(stderr, "Usage: %s file | %s\n", argv[0], player);
    exit(1);
  }

  // small optimisation to make it faster to print to stdout
  const int buffer_size = 65536;
  static char buffer[buffer_size];
  int ix = 0;

  PgState* state = init_state(argv[1]);

  enum YieldState status;

  status = get_sample(state);
  while (status == DataAvailable) {
    buffer[ix++] = state->v;
    if (ix == buffer_size) {
      fwrite(buffer, sizeof(char), buffer_size, stdout);
      ix = 0;
    }
    status = get_sample(state);
  }
  if (ix != 0) {
    fwrite(buffer, sizeof(char), ix, stdout);
  }

  cleanup(state);
}