use crate::core::{
    msvc::string::StdString,
    riot::{
        r3d::{asset_category::R3dAssetCategory, color::R3dColor, color_format::R3dColorFormat},
        renderer::texture::RiotRendererTexture,
        x3d::{base_texture::X3dBaseTexture, format::X3dFormat, texture::X3dTexture},
    },
};
use winapi::shared::minwindef::LPVOID;

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
    texture_ref_ptr: *mut RiotRendererTexture,
    texture: *mut X3dBaseTexture,
    texture_2d: *mut X3dTexture,
    asset_category: R3dAssetCategory,
    resource_pool_type: u32, // X3D:POOL
}

impl R3dTexture {
    pub fn flags(&self) -> u32 {
        self.flags
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn ref_count(&self) -> u32 {
        self.ref_count
    }
    pub fn file_name(&self) -> String {
        self.file_name.to_string()
    }
    pub fn file_path(&self) -> String {
        self.file_path.to_string()
    }
    pub fn hash_name(&self) -> String {
        self.hash_name.to_string()
    }
    pub fn pitch(&self) -> u32 {
        self.pitch
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn input_target_texture_format(&self) -> X3dFormat {
        self.input_target_texture_format
    }
    pub fn input_mip_map_count(&self) -> u32 {
        self.input_mip_map_count
    }
    pub fn input_key_color(&self) -> R3dColor {
        self.input_key_color
    }
    pub fn is_in_asset_category(&self) -> bool {
        self.in_asset_category
    }
    pub fn creation_flags(&self) -> u32 {
        self.creation_flags
    }
    pub fn texture_format(&self) -> X3dFormat {
        self.texture_format
    }
    pub fn color_format(&self) -> Option<&'static R3dColorFormat> {
        unsafe { self.color_format.as_ref() }
    }
    pub fn is_format_extended(&self) -> bool {
        self.is_format_extended
    }
    pub fn is_default_texture(&self) -> bool {
        self.is_default_texture
    }
    pub fn renderer_texture(&self) -> Option<&'static RiotRendererTexture> {
        unsafe {
            let texture = self.texture_ref_ptr.as_mut::<'static>();
            if let Some(texture) = texture {
                texture.add_ref();

                Some(&*texture)
            } else {
                None
            }
        }
    }
    pub fn texture(&self) -> *mut X3dBaseTexture {
        self.texture
    }
    pub fn texture_2d(&self) -> *mut X3dTexture {
        self.texture_2d
    }
}
