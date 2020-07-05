use crate::core::d3d9::direct3d9::IDirect3D9;
use imgui_dx9_renderer::IDirect3DDevice9;
use winapi::{
    shared::{d3d9::LPDIRECT3DDEVICE9, minwindef::LPVOID},
    um::winnt::VOID,
};

#[repr(C)]
#[derive(Debug)]
pub struct X3dD3d9Main {
    vtbl: LPVOID,
    ref_count: u32,
    d3d9_handle: LPDIRECT3DDEVICE9,
    cg_context: LPVOID, // _CGcontext*
}

impl X3dD3d9Main {
    pub fn d3d9_handle_mut(&mut self) -> Option<&'static mut IDirect3DDevice9> {
        unsafe { self.d3d9_handle.as_mut::<'static>() }
    }
}
