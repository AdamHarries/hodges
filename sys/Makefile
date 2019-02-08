CC=clang
FFMPEG_LIBS=    libavdevice                        \
                libavformat                        \
                libavfilter                        \
                libavcodec                         \
                libswresample                      \
                libswscale                         \
				libavutil \

CFLAGS += -O3 -march=native -Iinclude
CFLAGS := $(shell pkg-config --cflags $(FFMPEG_LIBS)) $(CFLAGS)
LDLIBS := $(shell pkg-config --libs $(FFMPEG_LIBS)) $(LDLIBS)

all: pcmdumpf

pcmdumpf: apps/pcmdumpf.c libhodges.o include/libhodges.h Makefile
	echo $(LDLIBS)
	$(CC) $(CFLAGS) $(LDLIBS) -o pcmdump apps/pcmdumpf.c libhodges.o

libhodges.o: src/libhodges.c include/libhodges.h Makefile
	$(CC) $(CFLAGS) -c src/libhodges.c

clean:
	rm -f libpcmdump.o
	rm -f pcmdumpf