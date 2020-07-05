use winapi::shared::minwindef::LPVOID;

#[repr(C)]
#[derive(Debug)]
pub struct StdList<T> {
    items: *mut T,
    size: usize,
}
