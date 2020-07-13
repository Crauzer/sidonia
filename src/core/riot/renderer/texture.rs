use std::ffi::{CStr, CString};
use winapi::{ctypes::c_char, shared::minwindef::LPVOID};

#[repr(C)]
#[derive(Debug)]
pub struct RiotRendererTexture {
    vtable: LPVOID,
    pooled_name: *const c_char,
    texture_type: RiotRendererTextureType,
    handle: LPVOID,
    ref_count: i32,
    render_target: bool,
    auto_destroy: bool,
}

#[repr(u32)]
#[derive(Debug)]
pub enum RiotRendererTextureType {
    k1D = 0,
    k2D = 1,
    k3D = 2,
    Cube = 3,
}

impl RiotRendererTexture {
    pub fn pooled_name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.pooled_name) }
    }

    pub fn add_ref(&mut self) {
        self.ref_count += 1;
    }
}

impl Drop for RiotRendererTexture {
    fn drop(&mut self) {
        self.ref_count -= 1;
    }
}
