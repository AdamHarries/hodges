pub mod util;

use util::impls::*;
use util::naive_estimator::*;

use std::collections::*;
use std::env;

use std::io::Write;

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

fn display_bpm(
    name: &str,
    expected: Option<f32>,
    error: &mut f32,
    bpm: f32,
) -> std::io::Result<()> {
    let bpm = match expected {
        Some(e) => {
            let pce = 100.0 * f32::abs(e - bpm) / e;
            *error += pce;
            pce
        }
        None => bpm,
    };
    print!(" / {}: {:3.2}", name, bpm);
    std::io::stdout().flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let trials = args[2].parse::<i32>().unwrap();
    let expected = if args.len() > 3 {
        Some(args[3].parse::<f32>().unwrap())
    } else {
        None
    };

    let mut estimator = if args.len() > 4 {
        if args[4] == "default" {
            Naive::default()
        } else if args[4] == "fine" {
            Naive::fine()
        } else if args[4] == "rough" {
            Naive::rough()
        } else {
            Naive::default()
        }
    } else {
        Naive::default()
    };

    println!("\nReading from file: {}", filename);

    let mut error: f32 = 0.0;

    println!(" === Trial: === ");
    for i in 0..trials {
        print!("\nT> {} ", i);
        std::io::stdout().flush()?;

        display_bpm(
            "h_fr",
            expected,
            &mut error,
            h_fr(filename.clone(), &mut estimator),
        )?;

        display_bpm(
            "h_br_ia",
            expected,
            &mut error,
            h_br_ia(filename.clone(), &mut estimator),
        )?;

        display_bpm(
            "h_br_ni",
            expected,
            &mut error,
            h_br_ni(filename.clone(), &mut estimator),
        )?;

        display_bpm(
            "f_fr",
            expected,
            &mut error,
            f_fr(filename.clone(), &mut estimator),
        )?;

        display_bpm(
            "f_fr_ia",
            expected,
            &mut error,
            f_fr_ia(filename.clone(), &mut estimator),
        )?;
    }

    flame::dump_stdout();

    // Report the error in aggregate
    println!(
        "Res> Error: (mean across all) --- {:3.2}",
        error / (trials as f32 * 5.0)
    );

    // Report metrics for the time taken for each method

    let spans = flame::spans();
    struct Stat {
        min: u64,
        max: u64,
        sum: u64,
    };
    let mut stats: BTreeMap<&str, Stat> = BTreeMap::new();

    for s in spans.iter() {
        let stat = stats.entry(&s.name).or_insert(Stat {
            min: s.delta,
            max: s.delta,
            sum: 0,
        });

        stat.sum += s.delta;

        if s.delta < stat.min {
            stat.min = s.delta;
        }

        if s.delta > stat.max {
            stat.max = s.delta;
        }
    }

    for (k, v) in stats.iter() {
        println!(
            "Res> Span: {:8} --- min: {:3.2} --- mean: {:3.2} --- max: {:3.2}",
            k,
            (v.min as f64) * 1e-6,
            (v.sum as f64) * 1e-6 * (1.0 / trials as f64),
            (v.max as f64) * 1e-6
        );
    }

    Ok(())
}
