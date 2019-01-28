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

pcmdump: libpcmdump.c # pcmdump.o libpcmdump.o
	$(CC) $(CFLAGS) $(LDLIBS) -o pcmdump libpcmdump.c # libpcmdump.o

libpcmdump.o: libpcmdump.c
	$(CC) $(CFLAGS) $(LDLIBS) -c libpcmdump.c

clean:
	rm -f pcmdump