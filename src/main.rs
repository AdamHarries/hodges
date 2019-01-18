pub mod audiobuffer;
pub mod reference;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    let buffer = reference::read_audio(filename);

    for value in buffer {
        dbg!(value);
    }
}
