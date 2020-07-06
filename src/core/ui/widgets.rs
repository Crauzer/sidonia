pub mod game_renderer;
pub mod game_renderer_camera;
pub mod game_renderer_stats;

pub trait Widget {
    fn render<'ui>(&mut self, ui: &'ui imgui::Ui);
}
