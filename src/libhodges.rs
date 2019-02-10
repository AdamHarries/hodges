#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;

// // std library imports
use std::ffi::CString;
use std::path::PathBuf;

use std::marker::PhantomData;

use std::ptr;
use std::slice;

pub struct State<T> {
    state_ptr: *mut std::os::raw::c_void,
    phantom: PhantomData<T>,
}

pub trait Source {
    type SampleT;
    fn get(&self) -> Result<Self::SampleT, YieldState>;
    fn get_buffer(&self) -> Result<&[Self::SampleT], YieldState>;
}

impl<T> State<T> {
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

impl Source for State<i8> {
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

    fn get_buffer(&self) -> Result<&[Self::SampleT], YieldState> {
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

impl Source for State<f32> {
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

    fn get_buffer(&self) -> Result<&[Self::SampleT], YieldState> {
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

impl Iterator for State<i8> {
    type Item = i8;

    fn next(&mut self) -> Option<i8> {
        match self.get() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl Iterator for State<f32> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        match self.get() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl<T> Drop for State<T> {
    fn drop(&mut self) -> () {
        unsafe {
            cleanup(self.state_ptr);
        }
    }
}
