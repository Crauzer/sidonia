use crate::core::{
    game::Game,
    riot::r3d::light_system::R3dLightSystem,
    ui::widgets::{
        camera::CameraWidget, game_renderer::GameRendererWidget, r3d_light::R3dLightWidget, r3d_light_system::R3dLightSystemWidget,
        r3d_sun::R3dSunWidget, simple_environment_asset::SimpleEnvironmentAssetWidget, OpenableWidget, OpenableWidgetState, Widget,
    },
    Core, CoreFetchData,
};
use imgui::Ui;

pub struct MainWindowWidget {
    signal_detach: bool,
    signal_reset: bool,
    game_renderer_widget: OpenableWidget<GameRendererWidget>,
    camera_widget: OpenableWidget<CameraWidget>,
    simple_environment_asset_widget: OpenableWidget<SimpleEnvironmentAssetWidget>,
    sun_widget: OpenableWidget<R3dSunWidget>,
    world_light_system_widget: OpenableWidget<R3dLightSystemWidget>,
}

impl MainWindowWidget {
    pub fn new() -> Self {
        MainWindowWidget {
            signal_detach: false,
            signal_reset: false,
            game_renderer_widget: OpenableWidget::new(GameRendererWidget::new()),
            camera_widget: OpenableWidget::new(CameraWidget::new()),
            simple_environment_asset_widget: OpenableWidget::new(SimpleEnvironmentAssetWidget::new()),
            sun_widget: OpenableWidget::new(R3dSunWidget::new()),
            world_light_system_widget: OpenableWidget::new(R3dLightSystemWidget::new()),
        }
    }

    pub fn fetch_core_data(&self) -> CoreFetchData {
        CoreFetchData {
            should_reset: self.signal_reset,
            should_exit: self.signal_detach,
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

        if let Some(world_light_system) = game.world_light_system_mut() {
            self.world_light_system_widget.widget().fetch_data(world_light_system);
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

        if let Some(world_light_system) = game.world_light_system() {
            self.world_light_system_widget.widget_mut().update(world_light_system);
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
                self.signal_detach = ui.button(im_str!("Detach"), [200.0, 30.0]);
                self.signal_reset = ui.button(im_str!("Reset"), [200.0, 30.0]);
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
                if ui.button(im_str!("World Light System"), [200.0, 30.0]) {
                    self.world_light_system_widget.flip_state();
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
                if self.world_light_system_widget.state() == OpenableWidgetState::Open {
                    self.world_light_system_widget.widget_mut().render(ui);
                }
            });
    }
}
