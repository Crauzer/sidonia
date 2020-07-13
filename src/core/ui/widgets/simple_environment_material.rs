use crate::core::{
    riot::simple_environment::{RiotSimpleEnvironmentMaterial, RiotSimpleEnvironmentMaterialFlags, RiotSimpleEnvironmentMaterialType},
    ui::widgets::{r3d_texture::R3dTextureWidget, simple_environment_channel::SimpleEnvironmentChannelWidget, Widget},
};
use imgui::Ui;
use std::ffi::CStr;

pub struct SimpleEnvironmentMaterialWidget {
    name: String,
    material_type: RiotSimpleEnvironmentMaterialType,
    flags: RiotSimpleEnvironmentMaterialFlags,
    channels: Vec<SimpleEnvironmentChannelWidget>,
    textures: Vec<R3dTextureWidget>,
}

impl SimpleEnvironmentMaterialWidget {
    pub fn new() -> Self {
        SimpleEnvironmentMaterialWidget {
            name: String::with_capacity(260),
            material_type: RiotSimpleEnvironmentMaterialType::Default,
            flags: RiotSimpleEnvironmentMaterialFlags::from_bits(0).unwrap(),
            channels: Vec::with_capacity(8),
            textures: Vec::with_capacity(8),
        }
    }

    pub fn fetch_data(&self, material: &mut RiotSimpleEnvironmentMaterial) {
        for i in 0..self.channels.len() {
            self.channels[i].fetch_data(&mut material.channels_mut()[i])
        }
    }

    pub fn update(&mut self, material: &RiotSimpleEnvironmentMaterial) {
        self.name = String::from(material.name());
        self.material_type = material.material_type();
        self.flags = material.flags();

        for i in 0..material.channels().len() {
            let mut channel_widget = SimpleEnvironmentChannelWidget::new(i);

            channel_widget.update(&material.channels()[i]);
            self.channels.push(channel_widget);
        }
        for i in 0..material.textures().len() {
            let mut texture_widget = R3dTextureWidget::new();

            if let Some(texture) = material.textures()[i] {
                texture_widget.update(texture);
            }

            self.textures.push(texture_widget);
        }
    }
}

impl Widget for SimpleEnvironmentMaterialWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        unsafe {
            let tree_node_id = self.name.as_bytes();
            let tree_node_id = imgui::ImStr::from_utf8_with_nul_unchecked(tree_node_id);
            let material_type = self.material_type;
            let flags = self.flags;
            let channels = &mut self.channels;
            let textures = &mut self.textures;
            imgui::TreeNode::new(tree_node_id).build(ui, || {
                ui.text(format!("Type: {:#?}", material_type));
                ui.text(format!("Flags: {:#?}", flags));

                imgui::TreeNode::new(im_str!("Channels")).build(ui, || {
                    for channel in channels {
                        channel.render(ui);
                    }
                });
                imgui::TreeNode::new(im_str!("Textures")).build(ui, || {
                    for i in 0..textures.len() {
                        imgui::TreeNode::new(&imgui::ImString::new(format!("{}", i))).build(ui, || {
                            textures[i].render(ui);
                        });
                    }
                });
            });
        }
    }
}
