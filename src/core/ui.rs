use crate::core::riot::r3d::render_layer::R3dRenderLayer;
use imgui::{DrawData, Window};
use std::error::Error;
use std::ptr::NonNull;
use std::mem;
use crate::core::game::Game;
use imgui_dx9_renderer::IDirect3DDevice9;
use crate::core::d3d9::direct3d9::IDirect3D9;
use crate::core::riot::x3d::d3d9::device::X3dD3d9Device;
use winapi::um::winuser::GetClientRect;
use std::mem::MaybeUninit;
use winapi::shared::d3d9types::D3DDEVICE_CREATION_PARAMETERS;
use winapi::shared::windef::RECT;
use crate::core::ui::input_manager::InputManager;

pub mod input_manager;

pub struct Ui {
    imgui_context: imgui::Context,
    imgui_renderer: Option<imgui_dx9_renderer::Renderer>,
    input_manager: Option<InputManager>
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            imgui_context: imgui::Context::create(),
            imgui_renderer: None,
            input_manager: None
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
                self.input_manager = Some(InputManager::new(&mut self.imgui_context, creation_parameters.hFocusWindow)
                    .expect("Failed to initialize imgui input manager"));

                log::info!("Initializing imgui renderer...");
                self.imgui_context.io_mut().display_size = [
                    (rect.right - rect.left) as f32,
                    (rect.bottom - rect.top) as f32,
                ];

                self.imgui_renderer = Some(imgui_dx9_renderer::Renderer::new(
                    &mut self.imgui_context,
                    NonNull::new(device).expect("Failed to create NonNull<IDirect3DDevice9>"))
                    .expect("Failed to initialize imgui renderer"));

                log::info!("imgui renderer initialized successfully");
            } else {
                log::error!("X3dD3d9Device.device was null");
            }
        }
    }

    pub fn update(&mut self, game: &mut Game, d3d9_device: &mut X3dD3d9Device) {
        match (game.is_renderer_initialized(), self.imgui_renderer.as_mut()) {
            // Game renderer is initialized so we also initialize our UI renderer
            (true, None) => {
                self.initialize_imgui(d3d9_device);
            },
            // Both the Game renderer and our renderer are initialized, do draw
            (true, Some(imgui_renderer)) => {
                let ui_frame = self.imgui_context.frame();

                imgui::Window::new(im_str!("Sidonia"))
                    .size([300.0, 300.0], imgui::Condition::FirstUseEver)
                    .build(&ui_frame, || {
                        ui_frame.text(im_str!("EndScene Hook"));
                        ui_frame.separator();
                        let mouse_pos = ui_frame.io().mouse_pos;
                        ui_frame.text(format!(
                            "Mouse Position: ({:.1},{:.1})",
                            mouse_pos[0], mouse_pos[1]
                        ));
                    });

                if let Some(game_clock) = game.game_clock() {
                    imgui::Window::new(im_str!("GameClock"))
                        .size([150.0, 150.0], imgui::Condition::FirstUseEver)
                        .build(&ui_frame, || {
                            ui_frame.text(format!("Is Initialized: {}", game_clock.is_initialized()));
                            ui_frame.separator();
                            ui_frame.text(format!("Current Time: {:.3}", game_clock.current_time()));
                            ui_frame.text(format!("Time Delta: {:.5}", game_clock.time_delta()));
                        });
                }

                imgui_renderer.render(ui_frame.render()).expect("Failed to render UI");
            }
            _ => {}
        }
    }
}