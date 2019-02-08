pub mod audiobuffer;
pub mod libhodges;
pub mod reference;

use std::env;
use libhodges::*; 

extern crate flame;
#[macro_use] extern crate flamer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    flame::start("read into buffer"); 
    let buffer = reference::read_audio(filename.clone());
    flame::end("read into buffer");

    flame::start("read with hodges");
    // get a vector from libhodges
    let mut hddat : Vec<f32> = Vec::with_capacity(1024 * 1024 * 1024); 

    // read into hdat from libhodges
    let state : libhodges::State<f32> = libhodges::State::from_file(filename).expect("Failed to open file with libhodges"); 
    let mut result = state.get(); 
    while let Ok(c) = result {
        hddat.push(c); 
        result = state.get(); 
    }
    flame::end("read with hodges");

    flame::start("compare");
    for (r, h) in buffer.zip(hddat.iter()) { 
        
        if r != *h { 
            println!("R : {}, H: {}  --- DISCREPANCY", r, *h); 
        }
    }
    flame::end("compare");

    flame::dump_stdout();
}
