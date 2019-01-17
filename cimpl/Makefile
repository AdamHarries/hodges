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

pcmdump: pcmdump.c
	$(CC) $(CFLAGS) $(LDLIBS) -o pcmdump pcmdump.c

speakerpipe-osx/speakerpipe:
	git clone git@github.com:richardkiss/speakerpipe-osx.git
	cd speakerpipe-osx
	$(MAKE)

clean:
	rm -rf *.dSYM
	rm -f decode
	rm -f remuxing
	rm -f decodeimage
	rm -f transcoding
	rm -f transcode_aac
	rm -f pcmdump