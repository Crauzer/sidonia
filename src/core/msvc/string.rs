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