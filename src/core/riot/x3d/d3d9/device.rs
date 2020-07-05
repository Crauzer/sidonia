use crate::core::riot::x3d::d3d9::main::X3dD3d9Main;
use winapi::shared::{
    d3d9::{IDirect3DDevice9, LPDIRECT3DDEVICE9},
    minwindef::LPVOID,
};

#[repr(C)]
#[derive(Debug)]
pub struct X3dD3d9Device {
    vtable: LPVOID,
    ref_count: u32,
    unk1: u32, // is_initialized ?
    main: *mut X3dD3d9Main,
    device_type: u32,
    device: LPDIRECT3DDEVICE9,
    adapter: u32,
}

impl X3dD3d9Device {
    pub fn device_mut(&self) -> Option<&'static mut IDirect3DDevice9> {
        unsafe { self.device.as_mut::<'static>() }
    }
}
