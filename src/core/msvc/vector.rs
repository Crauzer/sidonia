#[repr(C)]
#[derive(Debug)]
pub struct StdVector<T> {
    start: *mut T,
    end: *mut T,
    capacity: *mut T,
}
