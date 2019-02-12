#1/bin/bash

audio=$1

echo "ffmpeg"
time ffmpeg -loglevel quiet -i $audio -f f32le -acodec pcm_f32le -ac 1 -ar 44100 pipe:1  > /tmp/raw_ffmpeg.pcm
echo ""
echo "pcmdump"
time ./build/pcmdumpf $audio > /tmp/raw_pcmdump.pcm

diff /tmp/raw_ffmpeg.pcm /tmp/raw_pcmdump.pcm

if [ ! $? ] ; then
    echo "Output: $?"
    echo "Difference in output"
    xxd /tmp/raw_ffmpeg.pcm > /tmp/raw_ffmpeg.hex
    xxd /tmp/raw_pcmdump.pcm > /tmp/raw_pcmdump.hex
    diff /tmp/raw_ffmpeg.hex /tmp/raw_pcmdump.hex
fi
