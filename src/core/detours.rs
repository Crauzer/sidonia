use crate::core::{
    d3d9::direct3d9::IDirect3D9,
    riot::{
        logger::{RiotLogSeverityLevel, RiotLogTags, RiotLogger},
        x3d::d3d9::device::X3dD3d9Device,
    },
    utilities::memory,
};
use detour::static_detour;
use std::{error::Error, ffi, mem};
use winapi::{
    ctypes::{c_char, c_int},
    shared::{d3d9::LPDIRECT3DDEVICE9, minwindef::LPVOID},
};

// InitRenderer
static_detour! { pub static InitRendererHook: unsafe extern "thiscall" fn(LPVOID) -> c_int; }
type InitRenderer = unsafe extern "thiscall" fn(LPVOID) -> c_int;
type InitRendererFn = fn(LPVOID) -> c_int;

// EndScene
static_detour! { pub static RiotX3dD3D9DeviceEndSceneHook: unsafe extern "thiscall" fn(*mut X3dD3d9Device); }
type RiotX3dD3D9DeviceEndScene = unsafe extern "thiscall" fn(*mut X3dD3d9Device);
type RiotX3dD3D9DeviceEndSceneFn = fn(*mut X3dD3d9Device);

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
