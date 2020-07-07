use crate::core::{riot::camera_logic::RiotCameraLogicZoom, ui::widgets::Widget};
use imgui::Ui;

pub struct CameraZoomWidget {
    zoom_ease_time: f32,
    zoom_min_speed: f32,
    scale: f32,
    velocity: f32,
    current: f32,
    desired: f32,
}

impl CameraZoomWidget {
    pub fn new() -> Self {
        CameraZoomWidget {
            zoom_ease_time: 0.0,
            zoom_min_speed: 0.0,
            scale: 0.0,
            velocity: 0.0,
            current: 0.0,
            desired: 0.0,
        }
    }

    pub fn fetch_data(&self, camera_zoom: &mut RiotCameraLogicZoom) {
        camera_zoom.zoom_ease_time = self.zoom_ease_time;
        camera_zoom.zoom_min_speed = self.zoom_min_speed;
        camera_zoom.scale = self.scale;
        camera_zoom.velocity = self.velocity;
        camera_zoom.current = self.current;
        camera_zoom.desired = self.desired;
    }

    pub fn update(&mut self, camera_zoom: &RiotCameraLogicZoom) {
        self.zoom_ease_time = camera_zoom.zoom_ease_time;
        self.zoom_min_speed = camera_zoom.zoom_min_speed;
        self.scale = camera_zoom.scale;
        self.velocity = camera_zoom.velocity;
        self.current = camera_zoom.current;
        self.desired = camera_zoom.desired;
    }

    pub fn reset(&mut self) {
        *self = CameraZoomWidget::default();
    }
}

impl Default for CameraZoomWidget {
    fn default() -> Self {
        CameraZoomWidget {
            zoom_ease_time: 0.2,
            zoom_min_speed: 3.0,
            scale: 1.0,
            velocity: 0.0,
            current: 2250.0,
            desired: 2250.0,
        }
    }
}

impl Widget for CameraZoomWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        ui.input_float(im_str!("Zoom Ease Time"), &mut self.zoom_ease_time).build();
        ui.input_float(im_str!("Zoom Min Speed"), &mut self.zoom_min_speed).build();
        ui.input_float(im_str!("Scale"), &mut self.scale).build();
        ui.input_float(im_str!("Velocity"), &mut self.velocity).build();
        ui.input_float(im_str!("Current Zoom"), &mut self.current).build();
        ui.input_float(im_str!("Desired Zoom"), &mut self.desired).build();

        ui.separator();

        if ui.button(im_str!("Reset"), [150.0, 20.0]) {
            self.reset();
        }
    }
}
