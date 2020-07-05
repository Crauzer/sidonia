#[repr(C)]
pub struct RiotMutexId {
    sig: u32,
    opaque: [u8; 40],
}

#[repr(C)]
pub struct RiotMutex {
    id: RiotMutexId,
}
