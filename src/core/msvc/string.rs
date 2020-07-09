use bitflags::_core::fmt::Formatter;
use std::{
    error::Error,
    ffi::{CStr, CString},
    fmt,
    fmt::Display,
    os::raw::c_char,
};

#[repr(C)]
union Data {
    pointer: *mut u8,
    content: [u8; 16],
}

#[repr(C)]
pub struct StdString {
    data: Data,
    length: usize,
    capacity: usize,
}

impl fmt::Display for StdString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let string = if self.capacity < 16 {
                CStr::from_ptr(self.data.content.as_ptr() as *const c_char)
            } else {
                CStr::from_ptr(self.data.pointer as *const c_char)
            };

            write!(f, "{}", string.to_str().or_else(|err| Err(fmt::Error::default()))?);
        }

        Ok(())
    }
}

impl fmt::Debug for StdString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
