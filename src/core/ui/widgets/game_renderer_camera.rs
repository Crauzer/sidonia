use crate::core::{
    riot::r3d::{render_layer::R3dRenderLayer, vector3::R3dVector3},
    ui::widgets::Widget,
    utilities::math,
};
use glam::Mat4;
use imgui::Ui;

pub struct GameRendererCameraWidget {
    near_clip: f32,
    far_clip: f32,
    position: [f32; 3],
    rotation: [f32; 3],
}

impl GameRendererCameraWidget {
    pub fn new() -> Self {
        GameRendererCameraWidget {
            near_clip: 0.0,
            far_clip: 0.0,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
        }
    }

    fn reset_position(&mut self) {}

    pub fn fetch_data(&self, game_renderer: &mut R3dRenderLayer) {
        game_renderer.set_near_clip(self.near_clip);
        game_renderer.set_far_clip(self.far_clip);
        game_renderer.set_camera_position(R3dVector3::from(self.position));
    }

    pub fn update(&mut self, game_renderer: &R3dRenderLayer) {
        self.near_clip = game_renderer.camera_near_clip();
        self.far_clip = game_renderer.camera_far_clip();
        //self.position = game_renderer.camera_position().into();

        let camera_matrix: Mat4 = game_renderer.camera_matrix().into();
        log::info!("{:#?}", &camera_matrix);
        let camera_components = camera_matrix.to_scale_rotation_translation();

        self.position = camera_components.2.as_ref().clone();
        self.rotation = math::euler_from_quat(camera_components.1).as_ref().clone();
    }
}

impl Widget for GameRendererCameraWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        ui.drag_float(im_str!("Near Clip"), &mut self.near_clip)
            .min(0.0)
            .max(self.far_clip)
            .build();

        ui.drag_float(im_str!("Far Clip"), &mut self.far_clip)
            .min(self.near_clip)
            .max(25000.0)
            .build();

        ui.input_float3(im_str!("Position"), &mut self.position).build();
        ui.input_float3(im_str!("Rotation"), &mut self.rotation).build();
        //if ui.button(im_str!("Reset"), [150.0, 20.0]) {
        //
        //}
        ui.separator();
    }
}
