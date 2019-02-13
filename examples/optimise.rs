pub mod util;

use util::impls::*;
use util::naive_estimator::*;


use std::env;


use std::time::{ Instant};

extern crate flame;
#[macro_use]
extern crate flamer;

/*
    Compare the various ways that we expose access to ffmpeg.
    - h_fr (use hodges, read a single float at a time across the ffi)
    - h_br_ia (use hodges, read a buffer at a time, intermediate array)
    - h_br_ni (use hodges, read a buffer at a time, no intermediate array)
    - f_fr (use ffmpeg, read a single float at a time over pipe)
    - f_fr_ia (use ffmpeg, read all bytes to a buffer, intermediate array)

    Example usage:
        compare <audiofile> <trials>
*/

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let trials = args[2].parse::<i32>().unwrap();
    let expected = args[3].parse::<f32>().unwrap();

    println!("\nReading from file: {}", filename);

    println!("Interval, Steps, Samples, MeanBpm, MeanErrorA, MeanErrorB, MinTime, MeanTime");
    for mut estimator in Naive::large_range() {

        let (total_bpm, total_time, min_time, total_error) = (0..trials).fold((0.0,0,std::u128::MAX, 0.0), |(ba, ta, ma, ea), _ | {
            let now = Instant::now();
            let b = h_br_ni(filename.clone(), &mut estimator);
            // get a very rough elapsed time
            let t = now.elapsed().as_nanos();

            let e = (b - expected).abs() / expected;

            let nm = if t < ma {
                t
            }else {
                ma
            };

            (ba + b, ta + t, nm, ea + e)
        });

        let mean_bpm = total_bpm / trials as f32;
        let mean_time = 1e-6 * total_time as f32 / trials as f32;
        let min_time = min_time as f32 * 1e-6;
        let mean_error_a = 100.0 * total_error / trials as f32;
        let mean_error_b = 100.0 * (mean_bpm - expected).abs()/expected;

        println!("{:4}, {:4}, {:4}, {:6.2}, {:5.2}, {:5.2}, {:7.2}, {:7.2}", estimator.interval, estimator.steps, estimator.samples, mean_bpm, mean_error_a, mean_error_b, min_time, mean_time);

    }

    Ok(())
}
