use crate::core::msvc::string::StdString;
use crate::core::riot::x3d::format::X3dFormat;
use crate::core::riot::r3d::color::R3dColor;
use crate::core::riot::r3d::color_format::R3dColorFormat;
use winapi::shared::minwindef::LPVOID;
use crate::core::riot::x3d::base_texture::X3dBaseTexture;
use crate::core::riot::x3d::texture::X3dTexture;
use crate::core::riot::r3d::asset_category::R3dAssetCategory;

#[repr(C)]
pub struct R3dTexture {
    next: *mut R3dTexture,
    close: *mut R3dTexture,
    flags: u32,
    id: u32,
    ref_count: u32,
    file_name: StdString,
    file_path: StdString,
    hash_name: StdString,
    pitch: u32,
    width: u32,
    height: u32,
    input_target_texture_format: X3dFormat,
    input_mip_map_count: u32,
    input_key_color: R3dColor,
    in_asset_category: bool,
    creation_flags: u32,
    texture_format: X3dFormat,
    color_format: *const R3dColorFormat,
    is_format_extended: bool,
    is_default_texture: bool,
    texture_ref_ptr: LPVOID, // Riot::Renderer::TextureRPtr
    texture: *mut X3dBaseTexture,
    texture_2d: *mut X3dTexture,
    asset_category: R3dAssetCategory,
    resource_pool_type: u32, // X3D:POOL
}