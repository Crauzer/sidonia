use crate::core::{
    riot::camera_logic::{RiotCameraLogic, RiotCameraLogicMode},
    ui::widgets::{camera_attributes::CameraAttributesWidget, camera_zoom::CameraZoomWidget, Widget},
};
use imgui::Ui;
use num_traits::{FromPrimitive, ToPrimitive};

pub struct CameraWidget {
    mode: RiotCameraLogicMode,
    attributes: CameraAttributesWidget,
    zoom: CameraZoomWidget,
}

impl CameraWidget {
    pub fn new() -> Self {
        CameraWidget {
            mode: RiotCameraLogicMode::Topdown,
            attributes: CameraAttributesWidget::new(),
            zoom: CameraZoomWidget::new(),
        }
    }

    pub fn fetch_data(&self, camera_logic: &mut RiotCameraLogic) {
        camera_logic.set_mode(self.mode);
        self.attributes.fetch_data(camera_logic.attributes_mut());
        self.zoom.fetch_data(camera_logic.zoom_mut());
    }

    pub fn update(&mut self, camera_logic: &RiotCameraLogic) {
        let attributes = camera_logic.attributes();
        let zoom = camera_logic.zoom();

        self.mode = camera_logic.mode();
        self.attributes.update(attributes);
        self.zoom.update(zoom);
    }

    pub fn reset(&mut self) {
        *self = CameraWidget::default();
    }
}

impl Default for CameraWidget {
    fn default() -> Self {
        CameraWidget {
            mode: RiotCameraLogicMode::Topdown,
            attributes: CameraAttributesWidget::default(),
            zoom: CameraZoomWidget::default(),
        }
    }
}

impl Widget for CameraWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        imgui::Window::new(im_str!("Camera"))
            .size([500.0, 600.0], imgui::Condition::Appearing)
            .always_auto_resize(true)
            .build(&ui, || {
                let mut current_mode = self.mode.to_usize().or(Some(0)).unwrap();
                let modes = [im_str!("Topdown"), im_str!("FPS"), im_str!("TPS"), im_str!("Focus")];
                if imgui::ComboBox::new(im_str!("Mode")).build_simple_string(ui, &mut current_mode, &modes) {
                    self.mode = RiotCameraLogicMode::from_usize(current_mode)
                        .or(Some(RiotCameraLogicMode::Topdown))
                        .unwrap();
                }

                if imgui::CollapsingHeader::new(im_str!("Attributes")).default_open(false).build(&ui) {
                    self.attributes.render(&ui);
                }

                ui.separator();
                ui.spacing();

                if imgui::CollapsingHeader::new(im_str!("Zoom")).default_open(false).build(&ui) {
                    self.zoom.render(&ui);
                }

                ui.separator();

                if ui.button(im_str!("Reset"), [150.0, 20.0]) {
                    self.reset();
                }
            });
    }
}
