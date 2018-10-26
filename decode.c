#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#include <libavutil/frame.h>
#include <libavutil/mem.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>

// print out the steps and errors
static void logs(const char *fmt, ...);

void write_sample(float sample, float max_volume);

int main(int argc, char const *argv[])
{
    // Error things.
    const int err_str_len = 2048;
    char err_str[err_str_len];

    logs("Initialising decoding process.");
    logs("initialising codecs.");

    av_register_all();
    avcodec_register_all();

    const char *audio_filename = argv[1];
    logs("Input file: %s", audio_filename);

    // const AVCodec *codec;
    AVFormatContext *fmt_ctx = avformat_alloc_context();
    int err = 0;

    if ((err = avformat_open_input(&fmt_ctx, audio_filename, NULL, NULL)) != 0)
    {
        av_strerror(err, err_str, err_str_len);
        logs("Got error: %s", err_str);
        logs("Error, could not open file!");
        return -1;
    }
    logs("format %s, duration %lld us, bit_rate %lld", fmt_ctx->iformat->long_name, fmt_ctx->duration, fmt_ctx->bit_rate);

    // logs("Writing random data");

    // Write some random samples for now :)
    // for (;;)
    // {
    //     float value = (float)(rand() - rand()) / (float)(INT32_MAX);
    //     write_sample(value, 0.01);
    // }

    return 0;
}

// We're always going to want to decode our audio as f32 (for analysis).
// However, when we're outputting with out123, the default is to output s16.
// This function performs the conversion for us, and outputs the result.
void write_sample(float sample, float max_volume)
{
    uint16_t le = (uint16_t)(sample * 32767 * max_volume);
    fwrite(&le, 1, sizeof(le), stdout);
}

// logs implementation
static void logs(const char *fmt, ...)
{
    va_list args;
    fprintf(stderr, "LOG: ");
    va_start(args, fmt);
    vfprintf(stderr, fmt, args);
    va_end(args);
    fprintf(stderr, "\n");
}
