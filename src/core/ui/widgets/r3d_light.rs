use crate::core::{
    riot::{
        box3d::RiotBox3D,
        r3d::{
            light::{R3dLight, R3dLightType},
            texture::R3dTexture,
            vector3::R3dVector3,
        },
    },
    ui::widgets::Widget,
};
use imgui::Ui;
use num_traits::{FromPrimitive, ToPrimitive};
use std::ptr;
use winapi::shared::minwindef::LPVOID;

pub struct R3dLightWidget {
    position: [f32; 3],
    flags: u32,
    bounding_box: RiotBox3D,
    intensity: f32,
    light_type: R3dLightType,
    radius1: f32,
    radius2: f32,
    r_tmp: f32,
    light_system: LPVOID,
    color1: [f32; 3],
    color2: [f32; 3],
    direction: [f32; 3],
    spot_angle: f32,
    falloff_angle: f32,
    attribute1: f32,
    attribute2: f32,
    attribute3: f32,
    is_local_light: bool,
    shadow_index: i32,
    update_key: i32,
}

impl R3dLightWidget {
    pub fn new() -> Self {
        R3dLightWidget {
            position: [0.0, 0.0, 0.0],
            flags: 0,
            bounding_box: RiotBox3D::new(R3dVector3::zero(), R3dVector3::zero()),
            intensity: 0.0,
            light_type: R3dLightType::Point,
            radius1: 0.0,
            radius2: 0.0,
            r_tmp: 0.0,
            light_system: ptr::null_mut(),
            color1: [0.0, 0.0, 0.0],
            color2: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 0.0],
            spot_angle: 0.0,
            falloff_angle: 0.0,
            attribute1: 0.0,
            attribute2: 0.0,
            attribute3: 0.0,
            is_local_light: false,
            shadow_index: 0,
            update_key: 0,
        }
    }

    pub fn fetch_data(&self, light: &mut R3dLight) {
        light.position = R3dVector3::from(self.position);
        light.intensity = self.intensity;
        light.light_type = self.light_type;
        light.radius1 = self.radius1;
        light.radius2 = self.radius2;
        light.r = self.color1[0] * 255.0;
        light.g = self.color1[1] * 255.0;
        light.b = self.color1[2] * 255.0;
        light.r2 = self.color2[0] * 255.0;
        light.g2 = self.color2[1] * 255.0;
        light.b2 = self.color2[2] * 255.0;
        light.direction = R3dVector3::from(self.direction);
        light.spot_angle = self.spot_angle;
        light.falloff_angle = self.falloff_angle;
        light.attribute1 = self.attribute1;
        light.attribute2 = self.attribute2;
        light.attribute3 = self.attribute3;
        light.set_is_local_light(self.is_local_light);
    }

    pub fn update(&mut self, light: &R3dLight) {
        self.position = light.position.into();
        self.flags = light.flags();
        self.bounding_box = light.boounding_box();
        self.intensity = light.intensity;
        self.light_type = light.light_type;
        self.radius1 = light.radius1;
        self.radius2 = light.radius2;
        self.r_tmp = light.r_tmp();
        self.light_system = light.light_system();
        self.color1 = [light.r / 255.0, light.g / 255.0, light.b / 255.0];
        self.color2 = [light.r2 / 255.0, light.g2 / 255.0, light.b2 / 255.0];
        self.direction = light.direction.into();
        self.spot_angle = light.spot_angle;
        self.falloff_angle = light.falloff_angle;
        self.attribute1 = light.attribute1;
        self.attribute2 = light.attribute2;
        self.attribute3 = light.attribute3;
        self.is_local_light = light.is_local_light();
        self.shadow_index = light.shadow_index;
        self.update_key = light.update_key;
    }
}

impl Widget for R3dLightWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        ui.input_float3(im_str!("Position"), &mut self.position).build();
        ui.text(format!("Flags: {}", self.flags));

        if imgui::CollapsingHeader::new(im_str!("Bounding Box")).build(ui) {
            ui.input_float3(im_str!("Min"), self.bounding_box.min.as_mut())
                .read_only(true)
                .build();
            ui.input_float3(im_str!("Max"), self.bounding_box.max.as_mut())
                .read_only(true)
                .build();
        }

        ui.input_float(im_str!("Intensity"), &mut self.intensity).build();

        let mut current_type = self.light_type.to_usize().or(Some(0)).unwrap();
        let light_types = [im_str!("Point"), im_str!("Directional"), im_str!("Unknown")];
        if imgui::ComboBox::new(im_str!("Type")).build_simple_string(ui, &mut current_type, &light_types) {
            self.light_type = R3dLightType::from_usize(current_type).or(Some(R3dLightType::Point)).unwrap();
        }

        ui.input_float(im_str!("Inner Radius"), &mut self.radius1).build();
        ui.input_float(im_str!("Outer Radius"), &mut self.radius2).build();
        ui.input_float(im_str!("r_tmp"), &mut self.r_tmp).build();
        ui.text(format!("Light System: {:#?}", self.light_system));

        imgui::ColorEdit::new(im_str!("Color 1"), &mut self.color1)
            .picker(false)
            .small_preview(true)
            .build(ui);
        imgui::ColorEdit::new(im_str!("Color 2"), &mut self.color2)
            .picker(false)
            .small_preview(true)
            .build(ui);

        ui.input_float3(im_str!("Direction"), &mut self.direction).build();
        ui.input_float(im_str!("Spot Angle"), &mut self.spot_angle).build();
        ui.input_float(im_str!("Falloff Angle"), &mut self.falloff_angle).build();
        ui.input_float(im_str!("Attribute 1"), &mut self.attribute1).build();
        ui.input_float(im_str!("Attribute 2"), &mut self.attribute2).build();
        ui.input_float(im_str!("Attribute 3"), &mut self.attribute3).build();
        ui.checkbox(im_str!("Is Local"), &mut self.is_local_light);
        ui.text(format!("Shadow Index: {}", self.shadow_index));
        ui.text(format!("Update Key: {}", self.update_key));
    }
}

impl From<&R3dLight> for R3dLightWidget {
    fn from(light: &R3dLight) -> Self {
        let mut widget = R3dLightWidget::new();
        widget.update(light);
        widget
    }
}
