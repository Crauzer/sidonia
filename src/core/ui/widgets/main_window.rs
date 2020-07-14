use crate::core::{
    game::Game,
    ui::widgets::{
        camera::CameraWidget, game_renderer::GameRendererWidget, r3d_sun::R3dSunWidget,
        simple_environment_asset::SimpleEnvironmentAssetWidget, OpenableWidget, OpenableWidgetState, Widget,
    },
};
use imgui::Ui;

pub struct MainWindowWidget {
    game_renderer_widget: OpenableWidget<GameRendererWidget>,
    camera_widget: OpenableWidget<CameraWidget>,
    simple_environment_asset_widget: OpenableWidget<SimpleEnvironmentAssetWidget>,
    sun_widget: OpenableWidget<R3dSunWidget>,
}

impl MainWindowWidget {
    pub fn new() -> Self {
        MainWindowWidget {
            game_renderer_widget: OpenableWidget::new(GameRendererWidget::new()),
            camera_widget: OpenableWidget::new(CameraWidget::new()),
            simple_environment_asset_widget: OpenableWidget::new(SimpleEnvironmentAssetWidget::new()),
            sun_widget: OpenableWidget::new(R3dSunWidget::new()),
        }
    }

    pub fn fetch_data(&self, game: &mut Game) {
        if let Some(game_renderer) = game.renderer_mut() {
            self.game_renderer_widget.widget().fetch_data(game_renderer);
        }
        if let Some(hud_manager) = game.hud_manager_mut() {
            if let Some(camera_logic) = hud_manager.camera_logic_mut() {
                self.camera_widget.widget().fetch_data(camera_logic);
            }
        }
        if let Some(simple_environment_asset) = game.simple_environment_asset_mut() {
            self.simple_environment_asset_widget.widget().fetch_data(simple_environment_asset);
        }
        if let Some(sun) = game.sun_mut() {
            self.sun_widget.widget().fetch_data(sun);
        }
    }

    pub fn update(&mut self, game: &Game) {
        // Update Game Renderer Widget
        if let Some(game_renderer) = game.renderer() {
            self.game_renderer_widget.widget_mut().update(game_renderer);
        }

        // Update Hud Manager Widgets
        if let Some(hud_manager) = game.hud_manager() {
            // Update Camera Widget
            if let Some(camera_logic) = hud_manager.camera_logic() {
                self.camera_widget.widget_mut().update(camera_logic);
            }
        }

        // Update Simple Environment Widget
        if let Some(simple_envionment_asset) = game.simple_environment_asset() {
            self.simple_environment_asset_widget.widget_mut().update(simple_envionment_asset);
        }

        // Update Sun widget
        if let Some(sun) = game.sun() {
            self.sun_widget.widget_mut().update(sun);
        }
    }
}

impl Widget for MainWindowWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        imgui::Window::new(im_str!("Sidonia"))
            .size([300.0, 300.0], imgui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .build(&ui, || {
                ui.text(im_str!(r"   _____ _____ _____   ____  _   _ _____          "));
                ui.text(im_str!(r"  / ____|_   _|  __ \ / __ \| \ | |_   _|   /\    "));
                ui.text(im_str!(r" | (___   | | | |  | | |  | |  \| | | |    /  \   "));
                ui.text(im_str!(r"  \___ \  | | | |  | | |  | | . ` | | |   / /\ \  "));
                ui.text(im_str!(r"  ____) |_| |_| |__| | |__| | |\  |_| |_ / ____ \ "));
                ui.text(im_str!(r" |_____/|_____|_____/ \____/|_| \_|_____/_/    \_\"));
                ui.separator();

                if ui.button(im_str!("Renderer"), [200.0, 30.0]) {
                    self.game_renderer_widget.flip_state();
                }
                if ui.button(im_str!("Camera"), [200.0, 30.0]) {
                    self.camera_widget.flip_state();
                }
                if ui.button(im_str!("Simple Environment"), [200.0, 30.0]) {
                    self.simple_environment_asset_widget.flip_state();
                }
                if ui.button(im_str!("Sun"), [200.0, 30.0]) {
                    self.sun_widget.flip_state();
                }

                if self.game_renderer_widget.state() == OpenableWidgetState::Open {
                    self.game_renderer_widget.widget_mut().render(ui);
                }
                if self.camera_widget.state() == OpenableWidgetState::Open {
                    self.camera_widget.widget_mut().render(ui);
                }
                if self.simple_environment_asset_widget.state() == OpenableWidgetState::Open {
                    self.simple_environment_asset_widget.widget_mut().render(ui);
                }
                if self.sun_widget.state() == OpenableWidgetState::Open {
                    self.sun_widget.widget_mut().render(ui);
                }
            });
    }
}
