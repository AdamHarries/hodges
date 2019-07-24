//! A rusty interface to libffmpeg for fast(ish) audio decoding.
//!
//! libhodges provides a simple to use interface for decoding audio files to single channel 44100hz 32-bit little endian PCM data. In effect, it's static bindings to ffmpeg that allow us to achieve the equivalent of this one-line ffmpeg call:
//! ```
//! ffmpeg -loglevel quiet -i <filename> -f f32le -acodec pcm_f32le -ac 1 -ar 44100 pipe:1
//! ```
//! Using libhodges, however, means that the user doesn't have to worry about subprocesses, streams, or other fun operating system rubbish - instead, they get access to the ffmpeg internals. The one line command line call can instead replaced with the following:
//! ```
//! let state: State<u8> =
//!     State::from_file(filename.clone())?;

//! for c in state {
//!     io::stdout().write(&[c])?;
//! }

//! io::stdout().flush()?;
//! ```
//!

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod sys {

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

};

extern crate libc;

// // std library imports
use std::ffi::CString;
use std::path::PathBuf;

use std::marker::PhantomData;

use std::ptr;
use std::slice;

/// Hodges is build around a `State` struct. These are our reference to the inner workings of libffmpeg, and are used to query the c interface for whether or not data is available.
pub struct State<'a, T> {
    state_ptr: *mut std::os::raw::c_void,
    phantom: PhantomData<&'a T>,
}

/// The source trait provides an interface similar to
pub trait Source {
    type SampleT;
    fn get(&self) -> Result<Self::SampleT, YieldState>;
}

impl<'a, T> State<'a, T> {
    pub fn from_file<P: Into<PathBuf>>(filename: P) -> Option<Self> {
        // get the filename as a string, then a c string
        let cs_filename = filename
            .into()
            .to_str()
            .and_then(|filename| CString::new(filename).ok())?;

        unsafe {
            // try to open the file using the ffi
            let state_ptr = init_state(cs_filename.as_ptr());

            // Check to see if the file pointer is valid
            if state_ptr.is_null() {
                // debug!("Got null file pointer (bad filename, or bad tags?)");
                return None;
            }

            // Finally, return the state
            Some(State {
                state_ptr: state_ptr,
                phantom: PhantomData,
            })
        }
    }
}

impl<'a> Source for State<'a, i8> {
    type SampleT = i8;
    fn get(&self) -> Result<Self::SampleT, YieldState> {
        unsafe {
            // Get a character;
            let status = advance_char_iterator(self.state_ptr);
            if status == YieldState_DataAvailable {
                let c = get_char(self.state_ptr);
                Ok(c)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, &[i8]> {
    type SampleT = &'a [i8];

    fn get(&self) -> Result<Self::SampleT, YieldState> {
        unsafe {
            let mut buffer: *mut i8 = ptr::null_mut();
            let mut samples: i32 = 0;

            let status = get_char_buffer(self.state_ptr, &mut buffer, &mut samples);

            if status == YieldState_DataAvailable {
                let array: &[i8] = slice::from_raw_parts(buffer, samples as usize);
                Ok(array)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, u8> {
    type SampleT = u8;
    fn get(&self) -> Result<Self::SampleT, YieldState> {
        unsafe {
            // Get a character;
            let status = advance_char_iterator(self.state_ptr);
            if status == YieldState_DataAvailable {
                let c = get_char(self.state_ptr);
                Ok(c as u8)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, &[u8]> {
    type SampleT = &'a [u8];

    fn get(&self) -> Result<Self::SampleT, YieldState> {
        unsafe {
            let mut buffer: *mut i8 = ptr::null_mut();
            let mut samples: i32 = 0;

            let status = get_char_buffer(self.state_ptr, &mut buffer, &mut samples);

            if status == YieldState_DataAvailable {
                let array: &[u8] = slice::from_raw_parts(buffer as *mut u8, samples as usize);
                Ok(array)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, f32> {
    type SampleT = f32;
    fn get(&self) -> Result<Self::SampleT, YieldState> {
        unsafe {
            let status = advance_float_iterator(self.state_ptr);
            if status == YieldState_DataAvailable {
                let f = get_float(self.state_ptr);
                Ok(f)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, &[f32]> {
    type SampleT = &'a [f32];

    fn get(&self) -> Result<Self::SampleT, YieldState> {
        unsafe {
            let mut buffer: *mut f32 = ptr::null_mut();
            let mut samples: i32 = 0;

            let status = get_float_buffer(self.state_ptr, &mut buffer, &mut samples);

            if status == YieldState_DataAvailable {
                let array: &[f32] = slice::from_raw_parts(buffer, samples as usize);
                Ok(array)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Iterator for State<'a, i8> {
    type Item = i8;

    fn next(&mut self) -> Option<i8> {
        match self.get() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl<'a> Iterator for State<'a, u8> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.get() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl<'a> Iterator for State<'a, f32> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        match self.get() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl<'a> Iterator for State<'a, &[f32]> {
    type Item = &'a [f32];

    fn next(&mut self) -> Option<Self::Item> {
        match self.get() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl<'a, T> Drop for State<'a, T> {
    fn drop(&mut self) -> () {
        unsafe {
            cleanup(self.state_ptr);
        }
    }
}
