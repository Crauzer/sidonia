use winapi::shared::minwindef::LPVOID;

#[repr(C)]
pub struct X3dIUnknown {
    vtbl: LPVOID,
}
