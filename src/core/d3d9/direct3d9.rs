use winapi::shared::minwindef::LPVOID;

#[repr(C)]
pub struct IDirect3D9 {
    vtbl: LPVOID
}