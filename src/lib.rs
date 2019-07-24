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
//! # Naming
//!
//! Hodges is part of the `Ellington` project - a set of tools designed to make it easier for swing dance DJ's to automatically calculate the tempo of swing music. Each component of the project is named after a member of (or arranger for) Duke Ellington's band. Hodges is named after [Johnny Hodges](https://en.wikipedia.org/wiki/Johnny_Hodges), the lead alto sax player of Ellington's band, and one of the most skillful and talented alto sax players of the swing era.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

extern crate libc;

// // std library imports
use std::ffi::CString;
use std::path::PathBuf;

use std::marker::PhantomData;

use std::ptr;
use std::slice;

/// Hodges is built around a `State` struct. These are our reference to the inner workings of libffmpeg, and are used to query the c interface for whether or not data is available.
///
/// # Implementation
///
/// The `State` struct stores a reference to a `state_ptr` - an internal C data structure to our libffmpeg bindings that tracks our progress at decoding an audio file. Decoding is done according to a "pull" or "request" based model. The programmer requests data from the state, which checks whether or not data is available, and if not, obtains more. If no data is available, and more can be obtained, then the state will indicate to the programemr that the state is finished, and all further calls will fail.
///
/// The internal state of the decoder cannot be "rewound" - it is the programmers responsibility to cache whatever data is read for futher use.
///
/// # Source and Iterators
///
/// The `State` struct implements two traits which provide access to the samples: [`Source`] and [`Iterator`]. In essence, the two interfaces are the same, and the `Iterator` interface uses the `Source` interface internally. Both are provided, as the `Iterator` interface is somewhat more universal, while the `Source` interface provides a more specialised interface, along with more specific error messages to the user.
///
pub struct State<'a, T> {
    state_ptr: *mut std::os::raw::c_void,
    phantom: PhantomData<&'a T>,
}

/// The source trait provides an interface similar to an iterator, but with a `Result` output, rather than an `Option`, which gives us more indepth error messages when we try to get a sample.
pub trait Source {
    /// The type of sample that the `get` method might return. This allows us to customise the kinds of sample that the state might return, for example singular floating point samples, or entire buffers of samples.
    type SampleT;

    /// Retrieve a sample from the state, or an error message indicating that an error occurred.
    fn get(&self) -> Result<Self::SampleT, sys::YieldState>;
}

impl<'a, T> State<'a, T> {
    /// Initialise some ffmpeg state given the name of an audio file. At present, this is the only interface to hodges, and reading from mp3/flac/etc data directly is not supported.
    pub fn from_file<P: Into<PathBuf>>(filename: P) -> Option<Self> {
        // get the filename as a string, then a c string
        let cs_filename = filename
            .into()
            .to_str()
            .and_then(|filename| CString::new(filename).ok())?;

        unsafe {
            // try to open the file using the ffi
            let state_ptr = sys::init_state(cs_filename.as_ptr());

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
    /// A signed byte, 1/4 of a f32le sample
    type SampleT = i8;
    /// Get a single signed byte
    fn get(&self) -> Result<Self::SampleT, sys::YieldState> {
        unsafe {
            // Get a character;
            let status = sys::advance_char_iterator(self.state_ptr);
            if status == sys::YieldState_DataAvailable {
                let c = sys::get_char(self.state_ptr);
                Ok(c)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, &[i8]> {
    /// A buffer of signed bytes, in aggregate this should be (bitwise, module endianness) equivalent to a buffer of float f32le samples.
    type SampleT = &'a [i8];

    /// Get an entire buffer of signed bytes.
    fn get(&self) -> Result<Self::SampleT, sys::YieldState> {
        unsafe {
            let mut buffer: *mut i8 = ptr::null_mut();
            let mut samples: i32 = 0;

            let status = sys::get_char_buffer(self.state_ptr, &mut buffer, &mut samples);

            if status == sys::YieldState_DataAvailable {
                let array: &[i8] = slice::from_raw_parts(buffer, samples as usize);
                Ok(array)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, u8> {
    /// An unsigned byte, 1/4 of a f32le sample
    type SampleT = u8;
    /// Get a single unsigned byte
    fn get(&self) -> Result<Self::SampleT, sys::YieldState> {
        unsafe {
            // Get a character;
            let status = sys::advance_char_iterator(self.state_ptr);
            if status == sys::YieldState_DataAvailable {
                let c = sys::get_char(self.state_ptr);
                Ok(c as u8)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, &[u8]> {
    /// A buffer of unsigned bytes, in aggregate this should be (bitwise, module endianness) equivalent to a buffer of float f32le samples.
    type SampleT = &'a [u8];

    /// Get an entire buffer of unsigned bytes.
    fn get(&self) -> Result<Self::SampleT, sys::YieldState> {
        unsafe {
            let mut buffer: *mut i8 = ptr::null_mut();
            let mut samples: i32 = 0;

            let status = sys::get_char_buffer(self.state_ptr, &mut buffer, &mut samples);

            if status == sys::YieldState_DataAvailable {
                let array: &[u8] = slice::from_raw_parts(buffer as *mut u8, samples as usize);
                Ok(array)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, f32> {
    /// A single f32le sample
    type SampleT = f32;
    /// Get a single f32le sample
    fn get(&self) -> Result<Self::SampleT, sys::YieldState> {
        unsafe {
            let status = sys::advance_float_iterator(self.state_ptr);
            if status == sys::YieldState_DataAvailable {
                let f = sys::get_float(self.state_ptr);
                Ok(f)
            } else {
                Err(status)
            }
        }
    }
}

impl<'a> Source for State<'a, &[f32]> {
    /// A full buffer of f32le samples
    type SampleT = &'a [f32];

    /// Get a full buffer of samples, rather than a single sample.
    fn get(&self) -> Result<Self::SampleT, sys::YieldState> {
        unsafe {
            let mut buffer: *mut f32 = ptr::null_mut();
            let mut samples: i32 = 0;

            let status = sys::get_float_buffer(self.state_ptr, &mut buffer, &mut samples);

            if status == sys::YieldState_DataAvailable {
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
            sys::cleanup(self.state_ptr);
        }
    }
}
