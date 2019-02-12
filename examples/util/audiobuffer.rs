// The audiobuffer from Ellington.
use byteorder::*;

use std::io::Read;

pub struct AudioBuffer {}

#[allow(dead_code)]
impl AudioBuffer {
    #[flame]
    pub fn from_stream<T: Read>(mut stream: T) -> Vec<f32> {
        const SAMPLES: usize = 8192;

        let mut i8buffer = [0; SAMPLES];

        let mut f32buffer: [f32; SAMPLES / 4] = [0.0; SAMPLES / 4];

        let mut buffer: Vec<f32> = Vec::with_capacity(SAMPLES);

        loop {
            // read some samples into the buffer
            let read = stream.read(&mut i8buffer[..]);
            match read {
                Ok(bytes) => {
                    if bytes == 0 {
                        break;
                    } else {
                        // get that many bytes as a slice
                        let mut i8slice = &i8buffer[..bytes];
                        let r = i8slice.read_f32_into::<LittleEndian>(&mut f32buffer[..bytes / 4]);

                        match r {
                            Ok(_) => {
                                buffer.extend_from_slice(&f32buffer[..bytes / 4]);
                            }
                            Err(e) => panic!("Encountered convert error {:?}", e),
                        }
                    }
                }
                Err(error) => panic!("Encountered read error: {:?}", error),
            }
        }
        buffer
    }
}
