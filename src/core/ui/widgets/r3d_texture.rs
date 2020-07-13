use crate::core::{
    riot::{
        r3d::{color::R3dColor, texture::R3dTexture},
        x3d::format::X3dFormat,
    },
    ui::widgets::Widget,
};
use imgui::Ui;

pub struct R3dTextureWidget {
    flags: u32,
    id: u32,
    ref_count: u32,
    file_name: String,
    file_path: String,
    hash_name: String,
    pitch: u32,
    width: u32,
    height: u32,
    input_target_texture_format: X3dFormat,
    input_mip_map_count: u32,
    is_in_asset_category: bool,
    creation_flags: u32,
    texture_format: X3dFormat,
    is_format_extended: bool,
    is_default_texture: bool,
}

impl R3dTextureWidget {
    pub fn new() -> Self {
        R3dTextureWidget {
            flags: 0,
            id: 0,
            ref_count: 0,
            file_name: String::new(),
            file_path: String::new(),
            hash_name: String::new(),
            pitch: 0,
            width: 0,
            height: 0,
            input_target_texture_format: X3dFormat::Unknown,
            input_mip_map_count: 0,
            is_in_asset_category: false,
            creation_flags: 0,
            texture_format: X3dFormat::Unknown,
            is_format_extended: false,
            is_default_texture: false,
        }
    }

    pub fn update(&mut self, texture: &R3dTexture) {
        self.flags = texture.flags();
        self.id = texture.id();
        self.ref_count = texture.ref_count();
        self.file_name = texture.file_name();
        self.file_path = texture.file_path();
        self.hash_name = texture.hash_name();
        self.pitch = texture.pitch();
        self.width = texture.width();
        self.height = texture.height();
        self.input_target_texture_format = texture.input_target_texture_format();
        self.input_mip_map_count = texture.input_mip_map_count();
        self.is_in_asset_category = texture.is_in_asset_category();
        self.creation_flags = texture.creation_flags();
        self.texture_format = texture.texture_format();
        self.is_format_extended = texture.is_format_extended();
        self.is_default_texture = texture.is_default_texture();

        if let Some(tex) = texture.renderer_texture() {
            //log::info!("{:#?}", tex);
        }
    }
}

impl Widget for R3dTextureWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        ui.bullet_text(&imgui::ImString::new(format!("ID: {}", self.id)));
        ui.text(format!("Flags: {}", self.flags));
        ui.text(format!("Ref Count: {}", self.ref_count));
        ui.separator();
        ui.text_wrapped(&imgui::ImString::new(&self.file_name));
        ui.text_wrapped(&imgui::ImString::new(&self.file_path));
        ui.text_wrapped(&imgui::ImString::new(&self.hash_name));
        ui.separator();
        ui.text(format!("Pitch: {}", self.pitch));
        ui.text(format!("Dimensions: {}x{}", self.width, self.height));
        ui.separator();
        ui.text(format!("Input Target Texture Format: {:#?}", self.input_target_texture_format));
        ui.text(format!("Input Mip Map Count: {}", self.input_mip_map_count));
        ui.text(format!("Is in Asset Category: {}", self.is_in_asset_category));
        ui.text(format!("Creation Flags: {}", self.creation_flags));
        ui.separator();
        ui.text(format!("Format: {:#?}", self.texture_format));
        ui.text(format!("Is Format Extended: {}", self.is_format_extended));
        ui.text(format!("Is Default Texture: {}", self.is_default_texture));
    }
}
