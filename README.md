# hodges

Hodges is a small rust library build around libav that aims to provide high quality, high speed audio transcoding to "raw" f32le PCE audio. It is designed to be statically linked (with the exception of libAV components), in order to avoid system calls when using ffmpeg to transcode audio.

It's named after Johnny Hodges, an alto sax player well known for playing with Duke Ellington's big band.

## Future improvements

Hodges is currently quite slow at decoding flac and alac files (2x-5x slower than ffmpeg), for reasons that are not entirely clear. It may be due to a lack of parallelism in the core decoding "loop".

Hodges currently dynamically links the various components of libav that it relies on, and assumes that the user/developer already has libav/ffmpeg installed at build time. At some point it would be good to manually build libav/ffmpeg so that it can be statically linked, or so that hodges can be cross compiled for other platforms.