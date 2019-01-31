#include "libpcmdump.h"
#include "stdio.h"
int main(int argc, char** argv) {
  if (argc != 2) {
    static const char* player = "ffplay -f f32le -ar 44100 -ac 1 -";

    fprintf(stderr, "Usage: %s file | %s\n", argv[0], player);
    exit(1);
  }

  PgState* state = init_state(argv[1]);

  enum YieldState status;

  status = get_sample(state);
  while (status == DataAvailable) {
    fputc(state->v, stdout);
    status = get_sample(state);
  }

  cleanup(state);
}