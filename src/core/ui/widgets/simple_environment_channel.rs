use crate::core::{
    riot::{color::RiotColorValue, simple_environment::RiotSimpleEnvironmentChannel},
    ui::widgets::Widget,
};
use glam::{Mat4, Quat, Vec3};
use imgui::Ui;

pub struct SimpleEnvironmentChannelWidget {
    id: usize,
    color: [f32; 4],
    texture_name: String,
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl SimpleEnvironmentChannelWidget {
    pub fn new(id: usize) -> Self {
        SimpleEnvironmentChannelWidget {
            id,
            color: [0.0, 0.0, 0.0, 0.0],
            texture_name: String::with_capacity(260),
            position: Vec3::zero(),
            rotation: Quat::identity(),
            scale: Vec3::zero(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn fetch_data(&self, channel: &mut RiotSimpleEnvironmentChannel) {
        let mut color: [f32; 4] = self.color;
        for i in 0..4 {
            color[i] /= 255.0;
        }

        channel.set_color(RiotColorValue::from(color));
        channel.set_transform(Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position).into());
    }

    pub fn update(&mut self, channel: &RiotSimpleEnvironmentChannel) {
        self.color = channel.color().into();
        for i in 0..4 {
            self.color[i] *= 255.0;
        }
        self.texture_name = String::from(channel.texture_name());

        let (scale, rotation, position) = Mat4::from(channel.transform()).to_scale_rotation_translation();
        self.position = position;
        self.rotation = rotation;
        self.scale = scale;
    }
}

impl Widget for SimpleEnvironmentChannelWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        imgui::TreeNode::new(&imgui::ImString::from(format!("{}", self.id))).build(ui, || {
            imgui::ColorEdit::new(im_str!("Color"), &mut self.color)
                .alpha(true)
                .inputs(false)
                .preview(imgui::ColorPreview::Alpha)
                .build(ui);
            ui.separator();
            ui.bullet_text(im_str!("Transform"));
            ui.input_float3(im_str!("Position"), self.position.as_mut())
                //.read_only(true)
                .build();
            ui.input_float4(im_str!("Rotation"), self.rotation.as_mut())
                //.read_only(true)
                .build();
            ui.input_float3(im_str!("Scale"), self.scale.as_mut())
                //.read_only(true)
                .build();
        });
    }
}
