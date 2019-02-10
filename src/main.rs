pub mod audiobuffer;
pub mod audiostream;
pub mod libhodges;
pub mod naive_estimator;
pub mod reference;

use libhodges::*;
use naive_estimator::*;
use std::env;

extern crate flame;
#[macro_use]
extern crate flamer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    for n in &mut [Naive::default(), Naive::rough(), Naive::fine()] {
        println!("Naive estimator: {:?}", n);
        flame::start("read into buffer");
        let b1 = reference::read_audio(filename.clone());
        let bpm = n.analyse(b1);
        flame::end("read into buffer");
        println!("Bpm: {}", bpm);

        flame::start("read via stream into buffer");
        let bpm2 = reference::read_audio_stream(filename.clone(), n);
        flame::end("read via stream into buffer");
        println!("Bpm2: {}", bpm2);

        flame::start("read with hodges");
        let bpm3 = {
            let state: libhodges::State<f32> = libhodges::State::from_file(filename.clone())
                .expect("Failed to open file with libhodges");
            n.analyse(state)
        };
        flame::end("read with hodges");
        println!("Bpm3: {}", bpm3);

        flame::start("read buffer with hodges");
        let bpm4 = {
            let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024 * 1024);
            let state: libhodges::State<f32> = libhodges::State::from_file(filename.clone())
                .expect("Failed to open file with libhodges");

            while let Ok(buffer) = state.get_buffer() {
                vec.extend_from_slice(buffer);
            }

            n.analyse(vec.into_iter())
        };

        flame::end("read buffer with hodges");
        println!("Bpm4: {}", bpm4);

        flame::start("read and analyse buffer with hodges");
        let bpm5 = {
            let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024 * 1024);
            let state: libhodges::State<f32> = libhodges::State::from_file(filename.clone())
                .expect("Failed to open file with libhodges");

            while let Ok(buffer) = state.get_buffer() {
                vec.extend_from_slice(buffer);
            }

            n.analyse_vec(&vec)
        };

        flame::end("read and analyse buffer with hodges");
        println!("Bpm5: {}", bpm5);

        flame::dump_stdout();
        flame::clear();
    }
}
