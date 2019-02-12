pub mod util;
use hodges::*;
use util::naive_estimator::*;

use std::env;

extern crate flame;
#[macro_use]
extern crate flamer;

/*
    Calculate the bpm/tempo of an audio file using the naive estimator and hodges. If "direct" is given, read a single float at a time to the estimator, if "buffered" is given, read the whole audio file into an intermediate buffer (using the buffered interface), and then analyse it.

    Example usage:
        bpm <audiofile> direct
    or
        bpm <audiofile> buffered
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let method = args[2].clone();

    println!("\nReading from file: {}", filename);

    let mut estimator = Naive::fine();

    let bpm = if method == "direct" {
        let state: State<f32> =
            State::from_file(filename.clone()).expect("Failed to open file with libhodges");
        estimator.analyse(state)
    } else if method == "buffered" {
        let state: State<&[f32]> =
            State::from_file(filename.clone()).expect("Failed to open file with libhodges");
        let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024);

        while let Ok(buffer) = state.get() {
            vec.extend_from_slice(buffer);
        }

        estimator.analyse(vec.into_iter())
    } else {
        panic!("Couldn't recognise arg[2] - was expecting either 'direct' or 'buffered'");
    };

    println!("Calculated naive bpm: {}", bpm);
}
