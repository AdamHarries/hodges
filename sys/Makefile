CC=clang
FFMPEG_LIBS=    libavdevice                        \
                libavformat                        \
                libavfilter                        \
                libavcodec                         \
                libswresample                      \
                libswscale                         \
				libavutil \

CFLAGS += -O3
CFLAGS := $(shell pkg-config --cflags $(FFMPEG_LIBS)) $(CFLAGS)
LDLIBS := $(shell pkg-config --libs $(FFMPEG_LIBS)) $(LDLIBS)

all: pcmdump pipe

pcmdump: libpcmdump.c libpcmdump.h Makefile
	$(CC) $(CFLAGS) $(LDLIBS) -o pcmdump libpcmdump.c

pipe: pipe.c Makefile
	$(CC) $(CFLAGS) -o pipe pipe.c

clean:
	rm -f pcmdump