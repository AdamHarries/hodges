#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;

// // std library imports
use std::ffi::CString;
use std::path::PathBuf;

use std::marker::PhantomData;

struct HodgesState<T> {
    state_ptr: *mut std::os::raw::c_void,
    phantom: PhantomData<T>,
}

trait HodgesSource {
    type SampleT;
    fn get(&self) -> Result<Self::SampleT, YieldState>;
}

impl<T> HodgesState<T> {
    fn from_file<P: Into<PathBuf>>(filename: P) -> Option<Self> {
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
            Some(HodgesState {
                state_ptr: state_ptr,
                phantom: PhantomData,
            })
        }
    }
}

impl HodgesSource for HodgesState<i8> {
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

impl HodgesSource for HodgesState<f32> {
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

impl<T> Drop for HodgesState<T> {
    fn drop(&mut self) -> () {
        unsafe {
            cleanup(self.state_ptr);
        }
    }
}
