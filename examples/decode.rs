pub mod util;
use hodges::*;

use std::env;

use std::io::{self, Write};

extern crate flame;
#[macro_use]
extern crate flamer;

/*
    Decode an audio file such that it can be played by (e.g.) ffplay.

    Example usage:
        decode <audiofile> | ffplay -f f32le -ar 44100 -ac 1 -
*/
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    let state: State<u8> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");

    for c in state {
        io::stdout().write(&[c])?;
    }

    io::stdout().flush()?;
    Ok(())
}
