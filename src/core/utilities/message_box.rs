use std::{ffi::CString, ptr};
use winapi::um::winuser::{MessageBoxA, MB_ICONINFORMATION, MB_OK};

pub fn show_message(message: &str) {
    unsafe {
        let text = CString::new(message).unwrap();

        MessageBoxA(ptr::null_mut(), text.as_ptr(), ptr::null(), MB_OK | MB_ICONINFORMATION);
    }
}
