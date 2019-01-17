#1/bin/bash

audio="aintright.mp3"

echo "ffmpeg"
time ffmpeg -loglevel quiet -i $audio -f f32le -acodec pcm_f32le -ac 1 -ar 44100 pipe:1 > /dev/null
echo ""
echo "filtering_audio"
time ./filtering_audio $audio 2>/dev/null > /dev/null

diff raw_ffmpeg.pcm raw_filtering.pcm