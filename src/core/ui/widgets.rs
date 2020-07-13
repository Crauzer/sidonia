pub mod ai_hero;
pub mod camera;
pub mod camera_attributes;
pub mod camera_zoom;
pub mod game_renderer;
pub mod game_renderer_camera;
pub mod game_renderer_stats;
pub mod r3d_light;
pub mod r3d_sun;
pub mod r3d_texture;
pub mod simple_environment_asset;
pub mod simple_environment_channel;
pub mod simple_environment_material;

pub trait Widget {
    fn render<'ui>(&mut self, ui: &'ui imgui::Ui);
}
