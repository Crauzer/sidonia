pub mod ai_hero;
pub mod camera;
pub mod camera_attributes;
pub mod camera_zoom;
pub mod game_renderer;
pub mod game_renderer_camera;
pub mod game_renderer_stats;

pub trait Widget {
    fn render<'ui>(&mut self, ui: &'ui imgui::Ui);
}
