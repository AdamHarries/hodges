#1/bin/bash

audio="../../audio/rabbits.mp3"

echo "ffmpeg"
time ffmpeg -loglevel quiet -i $audio -f f32le -acodec pcm_f32le -ac 1 -ar 44100 pipe:1 > raw_ffmpeg.pcm
echo ""
echo "pcmdump"
time ./../pcmdump $audio 2>/dev/null > raw_pcmdump.pcm

diff raw_ffmpeg.pcm raw_pcmdump.pcm