
FFMPEG_LIBS=    libavdevice                        \
                libavformat                        \
                libavfilter                        \
                libavcodec                         \
                libswresample                      \
                libswscale                         \
				libavutil \

CFLAGS += -Wall -g
CFLAGS := $(shell pkg-config --cflags $(FFMPEG_LIBS)) $(CFLAGS)
LDLIBS := $(shell pkg-config --libs $(FFMPEG_LIBS)) $(LDLIBS)

decode: clean 
	clang $(CFLAGS) $(LDLIBS) -o decode decode.c

run: decode romance.mp3
	./decode romance.mp3 | out123 -c 1 

clean: 
	rm -f decode
