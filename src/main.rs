pub mod impls;
pub mod naive_estimator;
pub mod reference;
pub mod sources;

use naive_estimator::*;

use impls::hodges::*;

use std::fs::File;

use std::collections::HashMap;

extern crate flame;
#[macro_use]
extern crate flamer;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let dirname = args[1].clone();
    let dirname = "/Users/adam/projects/hodges/audio/tracks";

    let mut estimator = Naive::default();

    let mut calls = 0;
    for fr in std::fs::read_dir(dirname).unwrap() {
        let path = fr.unwrap().path();
        let filename = path.to_str().unwrap().to_string();
        // let filename = String::from("/home/adam/personal/hodges/audio/tracks/campus.mp3");

        println!("\nReading from file: {}", filename);
        // repeat our experiments
        let iterations = 1;
        for _i in 0..iterations {
            // for n in &mut [Naive::default(), Naive::rough(), Naive::fine()] {
            //     println!("Naive estimator: {:?}", n);
            {
                let bpm = reference::r_ia(filename.clone(), &mut estimator);
                println!("r_ia: {}", bpm);
            }
            {
                let bpm = reference::r_ib_ia(filename.clone(), &mut estimator);
                println!("r_ib_ia: {}", bpm);
            }
            {
                let bpm = dr_ia(filename.clone(), &mut estimator);
                println!("dr_ia bpm: {}", bpm);
            }
            {
                let bpm = dr_ib_ia(filename.clone(), &mut estimator);
                println!("dr_ib_ia bpm: {}", bpm);
            }
            {
                let bpm = dr_ib_va(filename.clone(), &mut estimator);
                println!("dr_ib_va bpm: {}", bpm);
            }
            {
                let bpm = br_ib_va(filename.clone(), &mut estimator);
                println!("br_ib_va bpm: {}", bpm);
            }
            {
                let bpm = br_ib_ia(filename.clone(), &mut estimator);
                println!("br_ib_ia bpm: {}", bpm);
            }
            calls += 1;
        }

        {
            let spans = flame::spans();
            let mut times: HashMap<&str, u64> = HashMap::new();
            let mut min_times: HashMap<&str, u64> = HashMap::new();

            for s in spans.iter() {
                let time = times.entry(&s.name).or_insert(0);
                *time += s.delta;

                let min_time = min_times.entry(&s.name).or_insert(s.delta);
                if s.delta < *min_time {
                    *min_time = s.delta;
                }
            }

            println!("=== Mean times: ===");
            for (k, v) in times.iter() {
                println!(
                    "span: {}, time: {}",
                    k,
                    (*v as f64) * 1e-6 * (1.0 / calls as f64)
                );
            }

            println!("=== Min times: ===");
            for (k, v) in min_times.iter() {
                println!("span: {}, time: {}", k, (*v as f64) * 1e-6);
            }
        }
        println!(" ------- ")
    }

    // flame::dump_stdout();

    // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
