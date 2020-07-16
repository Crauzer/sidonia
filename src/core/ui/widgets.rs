use bitflags::_core::ops::{Deref, DerefMut};

pub mod ai_hero;
pub mod camera;
pub mod camera_attributes;
pub mod camera_zoom;
pub mod game_renderer;
pub mod game_renderer_camera;
pub mod game_renderer_stats;
pub mod main_window;
pub mod r3d_light;
pub mod r3d_light_system;
pub mod r3d_sun;
pub mod r3d_texture;
pub mod simple_environment_asset;
pub mod simple_environment_channel;
pub mod simple_environment_material;

pub trait Widget {
    fn render<'ui>(&mut self, ui: &'ui imgui::Ui);
}

pub struct OpenableWidget<T: Widget> {
    state: OpenableWidgetState,
    widget: T,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpenableWidgetState {
    Open,
    Closed,
}

impl<T: Widget> OpenableWidget<T> {
    pub fn new(widget: T) -> Self {
        OpenableWidget {
            state: OpenableWidgetState::Closed,
            widget,
        }
    }

    pub fn state(&self) -> OpenableWidgetState {
        self.state
    }

    pub fn open(&mut self) {
        self.state = OpenableWidgetState::Open;
    }
    pub fn close(&mut self) {
        self.state = OpenableWidgetState::Closed;
    }
    pub fn flip_state(&mut self) {
        match self.state {
            OpenableWidgetState::Open => self.state = OpenableWidgetState::Closed,
            OpenableWidgetState::Closed => self.state = OpenableWidgetState::Open,
        }
    }

    pub fn widget(&self) -> &T {
        &self.widget
    }
    pub fn widget_mut(&mut self) -> &mut T {
        &mut self.widget
    }
}
