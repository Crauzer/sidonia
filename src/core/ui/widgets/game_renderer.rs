use crate::core::{
    riot::r3d::render_layer::R3dRenderLayer,
    ui::widgets::{game_renderer_camera::GameRendererCameraWidget, game_renderer_stats::GameRendererStatsWidget, Widget},
};
use imgui::Ui;

pub struct GameRendererWidget {
    stats_widget: GameRendererStatsWidget,
    camera_widget: GameRendererCameraWidget,
}

impl GameRendererWidget {
    pub fn new() -> Self {
        GameRendererWidget {
            stats_widget: GameRendererStatsWidget::new(),
            camera_widget: GameRendererCameraWidget::new(),
        }
    }

    pub fn update(&mut self, game_renderer: &R3dRenderLayer) {
        self.stats_widget.update(game_renderer.stats());
        self.camera_widget.update(game_renderer);
    }
}

impl Widget for GameRendererWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        imgui::Window::new(im_str!("gRenderer"))
            .size([200.0, 300.0], imgui::Condition::Appearing)
            .build(&ui, || {
                ui.separator();
                ui.spacing();

                if imgui::CollapsingHeader::new(im_str!("Stats")).default_open(false).build(&ui) {
                    self.stats_widget.render(&ui);
                }

                ui.separator();

                if imgui::CollapsingHeader::new(im_str!("Camera")).default_open(false).build(&ui) {
                    self.camera_widget.render(&ui);
                }
            });
    }
}
