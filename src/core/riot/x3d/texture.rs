use winapi::shared::minwindef::LPVOID;

#[repr(C)]
#[derive(Debug)]
pub struct X3dTexture {
    vtbl: LPVOID
}