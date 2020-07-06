use crate::core::{game::Game, riot::x3d::d3d9::device::X3dD3d9Device, ui::input_manager::InputManager};
use std::{mem::MaybeUninit, ptr::NonNull};
use winapi::{
    shared::{d3d9types::D3DDEVICE_CREATION_PARAMETERS, windef::RECT},
    um::winuser::GetClientRect,
};
use crate::core::ui::widgets::game_renderer_stats::GameRendererStatsWidget;
use crate::core::ui::widgets::Widget;

pub mod input_manager;
pub mod widgets;

pub struct Ui {
    imgui_context: imgui::Context,
    imgui_renderer: Option<imgui_dx9_renderer::Renderer>,
    input_manager: Option<InputManager>,

    game_renderer_stats: GameRendererStatsWidget,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            imgui_context: imgui::Context::create(),
            imgui_renderer: None,
            input_manager: None,

            game_renderer_stats: GameRendererStatsWidget::new()
        }
    }

    pub fn initialize_imgui(&mut self, d3d9_device: &mut X3dD3d9Device) {
        unsafe {
            if let Some(device) = d3d9_device.device_mut() {
                //Get Creation Parameters so we can get the Window handle
                let mut creation_parameters = MaybeUninit::<D3DDEVICE_CREATION_PARAMETERS>::zeroed();
                let mut rect = MaybeUninit::<RECT>::zeroed();

                device.GetCreationParameters(creation_parameters.as_mut_ptr());
                let creation_parameters = creation_parameters.assume_init();

                GetClientRect(creation_parameters.hFocusWindow, rect.as_mut_ptr());
                let rect = rect.assume_init();

                log::info!("Initializing imgui input manager..");
                self.input_manager = Some(
                    InputManager::new(&mut self.imgui_context, creation_parameters.hFocusWindow)
                        .expect("Failed to initialize imgui input manager"),
                );

                log::info!("Initializing imgui renderer...");
                self.imgui_context.io_mut().display_size = [(rect.right - rect.left) as f32, (rect.bottom - rect.top) as f32];

                self.imgui_renderer = Some(
                    imgui_dx9_renderer::Renderer::new(
                        &mut self.imgui_context,
                        NonNull::new(device).expect("Failed to create NonNull<IDirect3DDevice9>"),
                    )
                    .expect("Failed to initialize imgui renderer"),
                );

                log::info!("imgui renderer initialized successfully");
            } else {
                log::error!("X3dD3d9Device.device was null");
            }
        }
    }

    pub fn reset(&mut self) {
        self.imgui_renderer = None;
        self.input_manager = None;
    }

    pub fn is_imgui_initialized(&self) -> bool {
        self.imgui_renderer.is_some() && self.input_manager.is_some()
    }

    pub fn update(&mut self, game: &mut Game, d3d9_device: &mut X3dD3d9Device) {
        match (game.is_renderer_initialized(), self.is_imgui_initialized()) {
            // Game renderer is initialized so we also initialize our UI renderer
            (true, false) => {
                self.initialize_imgui(d3d9_device);
            }
            // Both the Game renderer and our renderer are initialized, do draw
            (true, true) => {
                let ui_frame = self.imgui_context.frame();

                imgui::Window::new(im_str!("Sidonia"))
                    .size([300.0, 300.0], imgui::Condition::FirstUseEver)
                    .build(&ui_frame, || {
                        ui_frame.text(im_str!("EndScene Hook"));
                        ui_frame.separator();
                        let mouse_pos = ui_frame.io().mouse_pos;
                        ui_frame.text(format!("Mouse Position: ({:.1},{:.1})", mouse_pos[0], mouse_pos[1]));
                    });

                if let Some(game_clock) = game.game_clock() {
                    imgui::Window::new(im_str!("gGameClock"))
                        .size([150.0, 180.0], imgui::Condition::FirstUseEver)
                        .build(&ui_frame, || {
                            ui_frame.text(format!("Is Initialized: {}", game_clock.is_initialized()));
                            ui_frame.separator();
                            ui_frame.text(format!("Current Time: {:.3}", game_clock.current_time()));
                            ui_frame.text(format!("Time Delta: {:.5}", game_clock.time_delta()));
                        });
                }


                let game_renderer_stats = &mut self.game_renderer_stats;
                if let Some(game_renderer) = game.renderer_mut() {
                    imgui::Window::new(im_str!("gRenderer"))
                        .size([150.0, 300.0], imgui::Condition::Appearing)
                        .build(&ui_frame, || {
                            ui_frame.text(format!("Is Initialized: {}", game_renderer.is_initialized()));
                            ui_frame.separator();
                            ui_frame.spacing();

                            if imgui::CollapsingHeader::new(im_str!("Stats"))
                                .default_open(true)
                                .build(&ui_frame)
                            {
                                let stats = game_renderer.stats();

                                game_renderer_stats.update(stats);
                                game_renderer_stats.render(&ui_frame);

                                //ui_frame.text(format!("Texture       Memory: {}", stats.texture_memory()));
                                //ui_frame.text(format!("Buffer        Memory: {}", stats.buffer_memory()));
                                //ui_frame.text(format!("Screen Buffer Memory: {}", stats.texture_memory()));
                                //ui_frame.text(format!("Material Change Count: {}", stats.material_change_count()));
                                //ui_frame.text(format!("Mode Changes    Count: {}", stats.mode_changes_count()));
                                //ui_frame.text(format!("Texture Changes Count: {}", stats.texture_changes_count()));
                                //ui_frame.text(format!("Tris Rendered   Count: {}", stats.triangles_rendered_count()));
                                //ui_frame.text(format!("Average Strip Length: {}", stats.average_strip_length()));
                                //ui_frame.text(format!("Draw Count: {}", stats.draw_count()));
                            }
                        });
                }

                let imgui_renderer = self.imgui_renderer.as_mut().unwrap();
                imgui_renderer.render(ui_frame.render()).expect("Failed to render UI");
            }
            _ => {}
        }
    }
}
