pub mod util;
use hodges::*;
use util::naive_estimator::*;

use std::env;

extern crate flame;
#[macro_use]
extern crate flamer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let method = args[2].clone();

    println!("\nReading from file: {}", filename);

    let mut estimator = Naive::fine();
    let state: State<f32> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");

    let bpm = if method == "direct" {
        estimator.analyse(state)
    } else if method == "buffered" {
        let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024);

        while let Ok(buffer) = state.get_buffer() {
            vec.extend_from_slice(buffer);
        }

        estimator.analyse(vec.into_iter())
    } else {
        panic!("Couldn't recognise arg[2] - was expecting either 'direct' or 'buffered'");
    };

    println!("Calculated naive bpm: {}", bpm);
}
