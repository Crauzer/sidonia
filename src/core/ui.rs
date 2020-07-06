use crate::core::{
    game::Game,
    riot::x3d::d3d9::device::X3dD3d9Device,
    ui::{
        input_manager::InputManager,
        widgets::{game_renderer::GameRendererWidget, Widget},
    },
};
use std::{mem::MaybeUninit, ptr::NonNull};
use winapi::{
    shared::{d3d9types::D3DDEVICE_CREATION_PARAMETERS, windef::RECT},
    um::winuser::GetClientRect,
};

pub mod input_manager;
pub mod widgets;

pub struct Ui {
    imgui_context: imgui::Context,
    imgui_renderer: Option<imgui_dx9_renderer::Renderer>,
    input_manager: Option<InputManager>,

    game_renderer: GameRendererWidget,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            imgui_context: imgui::Context::create(),
            imgui_renderer: None,
            input_manager: None,

            game_renderer: GameRendererWidget::new(),
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
            // Both the Game renderer and our renderer are initialized, do update
            (true, true) => {
                if let Some(game_renderer) = game.renderer_mut() {
                    self.game_renderer.update(game_renderer);
                }
            }
            _ => {}
        }
    }

    pub fn render(&mut self) {
        if self.is_imgui_initialized() {
            let ui = self.imgui_context.frame();

            // Main window
            imgui::Window::new(im_str!("Sidonia"))
                .size([300.0, 300.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("EndScene Hook"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!("Mouse Position: ({:.1},{:.1})", mouse_pos[0], mouse_pos[1]));
                });

            self.game_renderer.render(&ui);

            let imgui_renderer = self.imgui_renderer.as_mut().unwrap();
            imgui_renderer.render(ui.render()).expect("Failed to render UI");
        }
    }
}
