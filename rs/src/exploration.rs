extern crate ffmpeg;

use std::env;
use std::path::Path;

use ffmpeg::{codec, filter, format, frame, media};
use ffmpeg::{rescale, Rescale};

// We want to achieve:
// - format: f32le
// - codec: pcm_f32le
// - channels: 1
// - rate: 44100

fn main() {
    ffmpeg::init().unwrap();
    let codecID = ffmpeg::codec::Id::PCM_F32LE;

    println!("Codec id: {:?}", codecID);

    let codec: ffmpeg::codec::Audio = match ffmpeg::codec::decoder::find(codecID) {
        Some(c) => c,
        None => panic!("Error, can't find that codec!"),
    }
    .audio()
    .unwrap();

    println!("Found codec: {}, {}", codec.name(), codec.description());

    // let mut output = octx.add_stream(codec)?;
    // let mut encoder = output.codec().encoder().audio()?;

    // let channel_layout = codec
    //     .channel_layouts()
    //     .map(|cls| cls.best(decoder.channel_layout().channels()))
    //     .unwrap_or(ffmpeg::channel_layout::STEREO);

    // encoder.set_rate(decoder.rate() as i32);
    // encoder.set_channel_layout(channel_layout);
    // encoder.set_channels(channel_layout.channels());
    // encoder.set_format(
    //     codec
    //         .formats()
    //         .expect("unknown supported formats")
    //         .next()
    //         .unwrap(),
    // );
    // encoder.set_bit_rate(decoder.bit_rate());
    // encoder.set_max_bit_rate(decoder.max_bit_rate());

    // encoder.set_time_base((1, decoder.rate() as i32));
    // output.set_time_base((1, decoder.rate() as i32));

    // let encoder = encoder.open_as(codec)?;
    // output.set_parameters(&encoder);

    // let filter = filter(filter_spec, &decoder, &encoder)?;
}
