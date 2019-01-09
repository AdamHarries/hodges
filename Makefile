CC=gcc
FFMPEG_LIBS=    libavdevice                        \
                libavformat                        \
                libavfilter                        \
                libavcodec                         \
                libswresample                      \
                libswscale                         \
				libavutil \

CFLAGS +=  -g
CFLAGS := $(shell pkg-config --cflags $(FFMPEG_LIBS)) $(CFLAGS)
LDLIBS := $(shell pkg-config --libs $(FFMPEG_LIBS)) $(LDLIBS)

all: decode remuxing decodeimage

decode: clean 
	$(CC) $(CFLAGS) $(LDLIBS) -o decode decode.c

remuxing: clean 
	$(CC) $(CFLAGS) $(LDLIBS) -o remuxing remuxing.c

decodeimage: clean 
	$(CC) $(CFLAGS) $(LDLIBS) -o decodeimage decodeimage.c

# run: decode romance.mp3
# 	./decode romance.mp3 | out123

run: decode romance.mp3 speakerpipe-osx/speakerpipe
	./decode romance.mp3 | speakerpipe-osx/speakerpipe


speakerpipe-osx/speakerpipe: 
	git clone git@github.com:richardkiss/speakerpipe-osx.git
	cd speakerpipe-osx 
	$(MAKE)

clean: 
	rm -rf *.dSYM
	rm -f decode
	rm -f remuxing
	rm -f decodeimage