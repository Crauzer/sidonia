use winapi::um::winuser::{MessageBoxA, MB_ICONINFORMATION, MB_OK};
use std::ptr;
use std::ffi::CString;

pub fn show_message(message: &str) {
    unsafe {
        let text = CString::new(message).unwrap();

        MessageBoxA(ptr::null_mut(), text.as_ptr(), ptr::null(), MB_OK | MB_ICONINFORMATION);
    }
}