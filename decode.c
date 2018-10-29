#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include <libavutil/frame.h>
#include <libavutil/mem.h>

#include <inttypes.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// print out the steps and errors
static void logs(const char *fmt, ...);

static int decode_packet(AVPacket *packet, AVCodecContext *codec_ctx, AVFrame *frame);

void write_sample(float sample, float max_volume);

#define AV_TRY(command)                                                        \
  if ((err = command) != 0) {                                                  \
    av_strerror(err, err_str, err_str_len);                                    \
    logs(#command " returned string:\n\t%s\n", err_str);                       \
  }

int main(int argc, char const *argv[]) {
  // Error things.
  const int err_str_len = 2048;
  char err_str[err_str_len];

  logs("Initialising decoding process.");
  logs("initialising codecs.");

  // av_register_all();
  // avcodec_register_all();

  const char *audio_filename = argv[1];
  logs("Input file: %s", audio_filename);

  // const AVCodec *codec;
  AVFormatContext *fmt_ctx = avformat_alloc_context();
  int err = 0;

  AV_TRY(avformat_open_input(&fmt_ctx, audio_filename, NULL, NULL))

  logs("format %s, duration %lld us, bit_rate %lld",
       fmt_ctx->iformat->long_name, fmt_ctx->duration, fmt_ctx->bit_rate);

  logs("Finding stream info from the format.");

  AV_TRY(avformat_find_stream_info(fmt_ctx, NULL))

  logs("Get the codec etc.");

  AVCodec *codec = NULL;
  AVCodecParameters *codec_params = NULL;
  int audio_stream_index = -1;

  for (int i = 0; i < fmt_ctx->nb_streams; i++) {
    // Get local variables for the codec and params etc
    AVCodecParameters *local_codec_params = NULL;
    local_codec_params = fmt_ctx->streams[i]->codecpar;

    // log some information about it
    logs("Stream %d", i);
    logs("AVStream->time_base before open coded %d/%d",
         fmt_ctx->streams[i]->time_base.num,
         fmt_ctx->streams[i]->time_base.den);
    logs("AVStream->r_frame_rate before open coded %d/%d",
         fmt_ctx->streams[i]->r_frame_rate.num,
         fmt_ctx->streams[i]->r_frame_rate.den);
    logs("AVStream->start_time %" PRId64, fmt_ctx->streams[i]->start_time);
    logs("AVStream->duration %" PRId64, fmt_ctx->streams[i]->duration);

    logs("finding the proper decoder (CODEC)");
    AVCodec *local_codec = NULL;
    // find the decoder.
    local_codec = avcodec_find_decoder(local_codec_params->codec_id);
    if (local_codec == NULL) {
      logs("ERROR: Unsupported codec!!");
      return -1;
    }

    if (local_codec_params->codec_type == AVMEDIA_TYPE_AUDIO) {
      codec = local_codec;
      codec_params = local_codec_params;
      audio_stream_index = i;
      logs("Audio Codec: %d channels, sample rate %d",
           local_codec_params->channels, local_codec_params->sample_rate);
    } else if (local_codec_params->codec_type == AVMEDIA_TYPE_VIDEO) {
      logs("Video Codec: resolution %d x %d", local_codec_params->width,
           local_codec_params->height);
    }

    logs("\tCodec %s ID %d bit_rate %lld", local_codec->long_name,
         local_codec->id, codec_params->bit_rate);
  }

  logs("Picked stream %d.", audio_stream_index);

  AVCodecContext *codec_ctx = avcodec_alloc_context3(codec);
  if (!codec_ctx) {
    logs("Error: Failed to allocate memory for codec context");
    return -1;
  }

  // fill in parameters
  AV_TRY(avcodec_parameters_to_context(codec_ctx, codec_params))

  // initialise the AVCodecContext to use the codec
  AV_TRY(avcodec_open2(codec_ctx, codec, NULL))

  // allocate a frame and packet
  AVFrame *frame = av_frame_alloc();
  if (!frame) {
    logs("Failed to allocate memory for frame.");
  }
  AVPacket *packet = av_packet_alloc();
  if (!packet) {
    logs("Failed to allocate memory for packet.");
  }

  int response = 0;

  while (av_read_frame(fmt_ctx, packet) >= 0) {
    if (packet->stream_index == audio_stream_index) {
      response = decode_packet(packet, codec_ctx, frame); 
      if(response < 0)
        break; 
    }
    av_packet_unref(packet); 
  }

  logs("Releasing resources"); 
  avformat_close_input(&fmt_ctx); 
  avformat_free_context(fmt_ctx); 
  av_packet_free(&packet);
  av_frame_free(&frame);
  avcodec_free_context(&codec_ctx);

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
void write_sample(float sample, float max_volume) {
  uint16_t le = (uint16_t)(sample * 32767 * max_volume);
  fwrite(&le, 1, sizeof(le), stdout);
  // fwrite(&sample, 1, sizeof(sample), stdout); 
}

static int decode_packet(AVPacket *packet, AVCodecContext *codec_ctx, AVFrame *frame) 
{
  int response = avcodec_send_packet(codec_ctx, packet); 
  if(response < 0){ 
    logs("Error while sending a packet to the decoder: %s", av_err2str(response));
    return response; 
  }
  while (response >= 0) { 
    response = avcodec_receive_frame(codec_ctx, frame);
    if(response == AVERROR(EAGAIN) || response == AVERROR_EOF) { 
      break; 
    } else if (response < 0) { 
      logs("Error while receiving a frame from the decoder: %s", av_err2str(response));
      return response;
    }

    int data_size = av_get_bytes_per_sample(codec_ctx->sample_fmt);
    if(data_size <0) { 
      logs("Error, failed to calculate data size!"); 
      exit(1);
    }

    float sample; 
    for(int i = 0; i < frame -> nb_samples; i++)
    {
      for(int ch = 0; ch < codec_ctx -> channels; ch++) {
        memcpy(&sample, frame->data[ch] + data_size*i, (size_t)data_size);
        write_sample(sample, 0.1);
      }
    }
  }

  return 0; 
}

// logs implementation
static void logs(const char *fmt, ...) {
  va_list args;
  fprintf(stderr, "LOG: ");
  va_start(args, fmt);
  vfprintf(stderr, fmt, args);
  va_end(args);
  fprintf(stderr, "\n");
}
