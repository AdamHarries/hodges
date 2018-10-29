#1/bin/bash

ffmpeg -loglevel quiet -i romance.mp3 -f f32le -acodec pcm_f32le -ac 1 -ar 44100 pipe:1 > raw_ffmpeg.pcm