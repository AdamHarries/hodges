pub mod util;

use util::impls::*;
use util::naive_estimator::*;

use std::collections::*;
use std::env;

extern crate flame;
#[macro_use]
extern crate flamer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let trials = args[2].parse::<i32>().unwrap();

    println!("\nReading from file: {}", filename);
    for estimator in &mut [Naive::default()] {
        // , Naive::rough(), Naive::fine()] {
        // repeat our experiments
        // println!("Estimator settings: {:#?}", estimator);
        let mut f = 0.0;
        println!(" === Trial: === ");
        for i in 0..trials {
            print!("T> {} / ", i);
            f += h_fr(filename.clone(), estimator);
            f += h_br_ia(filename.clone(), estimator);
            f += f_fr(filename.clone(), estimator);
            f += f_fr_ia(filename.clone(), estimator);
            println!("{}", f);
        }
        println!("Using result: {}", f);
        // Print some results.

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
                "Span: {:8} --- min: {:3.2} --- mean: {:3.2} --- max: {:3.2}",
                k,
                (v.min as f64) * 1e-6,
                (v.sum as f64) * 1e-6 * (1.0 / trials as f64),
                (v.max as f64) * 1e-6
            );
        }
        flame::clear();
    }
}
