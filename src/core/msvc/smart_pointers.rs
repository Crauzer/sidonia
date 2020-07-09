use winapi::shared::minwindef::LPVOID;

#[repr(C)]
pub struct StdSharedPtr<T> {
    ptr: *mut T,
    cntrl: *mut SharedWeakCount,
}

#[repr(C)]
pub struct StdWeakPtr<T> {
    ptr: *mut T,
    cntrl: *mut SharedWeakCount,
}

#[repr(C)]
struct SharedWeakCount {
    vtable: LPVOID,
    shared_owners: i32,
    shared_weak_owners: i32,
}
