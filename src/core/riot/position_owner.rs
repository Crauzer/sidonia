use winapi::shared::minwindef::LPVOID;

#[repr(C)]
#[derive(Debug)]
pub struct RiotIPositionOwner {
    vtable: LPVOID,
}
