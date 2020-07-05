use crate::core::{riot::x3d::d3d9::device::X3dD3d9Device, utilities::memory};
use detour::static_detour;
use std::{error::Error, mem};
use winapi::{
    ctypes::c_int,
    shared::{d3d9::IDirect3DDevice9, d3d9types::D3DPRESENT_PARAMETERS, minwindef::LPVOID},
};

// InitRenderer
static_detour! { pub static InitRendererHook: unsafe extern "thiscall" fn(LPVOID) -> c_int; }
type InitRenderer = unsafe extern "thiscall" fn(LPVOID) -> c_int;
type InitRendererFn = fn(LPVOID) -> c_int;

// EndScene
static_detour! { pub static RiotX3dD3D9DeviceEndSceneHook: unsafe extern "thiscall" fn(*mut X3dD3d9Device); }
type RiotX3dD3D9DeviceEndScene = unsafe extern "thiscall" fn(*mut X3dD3d9Device);
type RiotX3dD3D9DeviceEndSceneFn = fn(*mut X3dD3d9Device);

// Reset Device
static_detour! { pub static RiotX3dD3d9DeviceResetHook: unsafe extern "fastcall" fn(*mut IDirect3DDevice9, *mut D3DPRESENT_PARAMETERS); }
type RiotX3dD3d9DeviceReset = unsafe extern "thiscall" fn(*mut IDirect3DDevice9, *mut D3DPRESENT_PARAMETERS);
type RiotX3dD3d9DeviceResetFn = fn(*mut IDirect3DDevice9, *mut D3DPRESENT_PARAMETERS);

pub unsafe fn initialize_init_renderer_hook(detour: InitRendererFn) -> Result<(), Box<dyn Error>> {
    let address: InitRenderer = mem::transmute(memory::convert_file_offset_to_ptr(0x005756A0));

    InitRendererHook.initialize(address, detour)?.enable()?;

    Ok(())
}

pub unsafe fn initialize_riot_x3d_d3d9_device_end_scene_hook(detour: RiotX3dD3D9DeviceEndSceneFn) -> Result<(), Box<dyn Error>> {
    let address = mem::transmute(memory::convert_file_offset_to_ptr(0x009C2500));

    RiotX3dD3D9DeviceEndSceneHook.initialize(address, detour)?.enable()?;

    Ok(())
}

pub unsafe fn initialize_riot_x3d_d3d9_device_reset_hook(detour: RiotX3dD3d9DeviceResetFn) -> Result<(), Box<dyn Error>> {
    let address = mem::transmute(memory::convert_file_offset_to_ptr(0x009C4F50));

    RiotX3dD3d9DeviceResetHook.initialize(address, detour)?.enable()?;

    Ok(())
}
