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

all: pcmdump

pcmdump: pcmdump.c libpcmdump.o libpcmdump.h Makefile
	$(CC) $(CFLAGS) $(LDLIBS) -o pcmdump pcmdump.c libpcmdump.o

libpcmdump.o: libpcmdump.c libpcmdump.h Makefile
	$(CC) $(CFLAGS) -c libpcmdump.c

clean:
	rm -f pcmdump