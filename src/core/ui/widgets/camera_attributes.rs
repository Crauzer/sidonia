use crate::core::{
    riot::{camera_logic::RiotCameraLogicAttributes, r3d::vector3::R3dVector3},
    ui::widgets::Widget,
};
use imgui::Ui;

pub struct CameraAttributesWidget {
    world_position: [f32; 3],
    camera_freeze_point: [f32; 3],
    current_velocity: f32,
    current_pitch: f32,
    current_yaw: f32,
    start_yaw: f32,
    start_pitch: f32,
    current_fov: f32,
    last_click_map_time: f32,
}

impl CameraAttributesWidget {
    pub fn new() -> Self {
        CameraAttributesWidget {
            world_position: [0.0, 0.0, 0.0],
            camera_freeze_point: [0.0, 0.0, 0.0],
            current_velocity: 0.0,
            current_pitch: 0.0,
            current_yaw: 0.0,
            start_yaw: 0.0,
            start_pitch: 0.0,
            current_fov: 0.0,
            last_click_map_time: 0.0,
        }
    }

    pub fn fetch_data(&self, attributes: &mut RiotCameraLogicAttributes) {
        attributes.world_position = R3dVector3::from(self.world_position);
        attributes.camera_freeze_point = R3dVector3::from(self.camera_freeze_point);
        attributes.current_velocity = self.current_velocity;
        attributes.current_pitch = self.current_pitch;
        attributes.current_yaw = self.current_yaw;
        attributes.start_yaw = self.start_yaw;
        attributes.start_pitch = self.start_pitch;
        attributes.current_fov = self.current_fov;
        attributes.last_click_map_time = self.last_click_map_time;
    }

    pub fn update(&mut self, attributes: &RiotCameraLogicAttributes) {
        self.world_position = attributes.world_position.into();
        self.camera_freeze_point = attributes.camera_freeze_point.into();
        self.current_velocity = attributes.current_velocity;
        self.current_pitch = attributes.current_pitch;
        self.current_yaw = attributes.current_yaw;
        self.start_yaw = attributes.start_yaw;
        self.start_pitch = attributes.start_pitch;
        self.current_fov = attributes.current_fov;
        self.last_click_map_time = attributes.last_click_map_time;
    }
}

impl Widget for CameraAttributesWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        ui.input_float3(im_str!("World Position"), &mut self.world_position).build();
        ui.input_float3(im_str!("Freeze Point"), &mut self.camera_freeze_point).build();
        ui.separator();
        ui.input_float(im_str!("Velocity"), &mut self.current_velocity).build();
        ui.drag_float(im_str!("Pitch"), &mut self.current_pitch).min(0.0).max(360.0).build();
        ui.drag_float(im_str!("Yaw"), &mut self.current_yaw).min(0.0).max(360.0).build();
        ui.drag_float(im_str!("Start Pitch"), &mut self.start_pitch)
            .min(0.0)
            .max(360.0)
            .build();
        ui.drag_float(im_str!("Start Yaw"), &mut self.start_yaw).min(0.0).max(360.0).build();
        ui.drag_float(im_str!("FOV"), &mut self.current_fov).min(0.0).max(180.0).build();
        ui.separator();
        ui.text(format!("Last Click Map Time: {}", self.last_click_map_time));
    }
}
