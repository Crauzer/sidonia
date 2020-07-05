use crate::core::riot::x3d::surface::X3dISurface;
use crate::core::riot::x3d::texture::X3dTexture;
use crate::core::riot::r3d::texture::R3dTexture;
use winapi::ctypes::c_char;
use crate::core::riot::x3d::format::X3dFormat;
use winapi::shared::minwindef::LPVOID;

#[repr(C)]
#[derive(Debug)]
pub struct R3dScreenBuffer {
    vtbl: LPVOID,
    our_back_buffer: *mut X3dTexture,
    our_depth_buffer: *mut X3dISurface,
    surface1: *mut X3dISurface,
    texture1: *mut R3dTexture,
    texture2: *mut R3dTexture,
    have_z: u32,
    use_system_z: u32,
    cube_map: u32,
    buffer_format: X3dFormat,
    buffer_id: u32,
    usage: u32,
    view_width: u32,
    view_height: u32,
    width: u32,
    height: u32,
    debug_location: *const c_char,
    flags: u32
}