use crate::core::{
    game::Game,
    riot::x3d::d3d9::device::X3dD3d9Device,
    ui::{
        input_manager::InputManager,
        widgets::{
            camera::CameraWidget, game_renderer::GameRendererWidget, main_window::MainWindowWidget, r3d_sun::R3dSunWidget,
            simple_environment_asset::SimpleEnvironmentAssetWidget, Widget,
        },
    },
    Core, CoreFetchData,
};
use imgui::sys::ImVec4;
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

    main_window: MainWindowWidget,
}

impl Ui {
    pub fn new() -> Self {
        let mut imgui_context = imgui::Context::create();

        Ui::apply_theme(&mut imgui_context);

        Ui {
            imgui_context,
            imgui_renderer: None,
            input_manager: None,

            main_window: MainWindowWidget::new(),
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

    pub fn fetch_game_data(&self, game: &mut Game) {
        self.main_window.fetch_data(game);
    }

    pub fn fetch_core_data(&self) -> CoreFetchData {
        self.main_window.fetch_core_data()
    }

    pub fn update(&mut self, game: &Game, d3d9_device: &mut X3dD3d9Device) {
        match (game.is_renderer_initialized(), self.is_imgui_initialized()) {
            // Game renderer is initialized so we also initialize our UI renderer
            (true, false) => {
                self.initialize_imgui(d3d9_device);
            }
            // Both the Game renderer and our renderer are initialized, do update
            (true, true) => {
                self.main_window.update(game);
            }
            _ => {}
        }
    }

    pub fn render(&mut self) {
        if self.is_imgui_initialized() {
            let ui = self.imgui_context.frame();

            self.main_window.render(&ui);

            let imgui_renderer = self.imgui_renderer.as_mut().unwrap();
            imgui_renderer.render(ui.render()).expect("Failed to render UI");
        }
    }

    // https://github.com/ocornut/imgui/issues/707#issuecomment-512669512
    fn apply_theme(context: &mut imgui::Context) {
        let style = context.style_mut();

        style.frame_rounding = 4.0;
        style.grab_rounding = 4.0;

        let colors = &mut style.colors;

        colors[imgui::StyleColor::Text as usize] = ImVec4::new(0.95, 0.96, 0.98, 1.00).into();
        colors[imgui::StyleColor::Text as usize] = ImVec4::new(0.95, 0.96, 0.98, 1.00).into();
        colors[imgui::StyleColor::TextDisabled as usize] = ImVec4::new(0.36, 0.42, 0.47, 1.00).into();
        colors[imgui::StyleColor::WindowBg as usize] = ImVec4::new(0.11, 0.15, 0.17, 1.00).into();
        colors[imgui::StyleColor::ChildBg as usize] = ImVec4::new(0.15, 0.18, 0.22, 1.00).into();
        colors[imgui::StyleColor::PopupBg as usize] = ImVec4::new(0.08, 0.08, 0.08, 0.94).into();
        colors[imgui::StyleColor::Border as usize] = ImVec4::new(0.08, 0.10, 0.12, 1.00).into();
        colors[imgui::StyleColor::BorderShadow as usize] = ImVec4::new(0.00, 0.00, 0.00, 0.00).into();
        colors[imgui::StyleColor::FrameBg as usize] = ImVec4::new(0.20, 0.25, 0.29, 1.00).into();
        colors[imgui::StyleColor::FrameBgHovered as usize] = ImVec4::new(0.12, 0.20, 0.28, 1.00).into();
        colors[imgui::StyleColor::FrameBgActive as usize] = ImVec4::new(0.09, 0.12, 0.14, 1.00).into();
        colors[imgui::StyleColor::TitleBg as usize] = ImVec4::new(0.09, 0.12, 0.14, 0.65).into();
        colors[imgui::StyleColor::TitleBgActive as usize] = ImVec4::new(0.08, 0.10, 0.12, 1.00).into();
        colors[imgui::StyleColor::TitleBgCollapsed as usize] = ImVec4::new(0.00, 0.00, 0.00, 0.51).into();
        colors[imgui::StyleColor::MenuBarBg as usize] = ImVec4::new(0.15, 0.18, 0.22, 1.00).into();
        colors[imgui::StyleColor::ScrollbarBg as usize] = ImVec4::new(0.02, 0.02, 0.02, 0.39).into();
        colors[imgui::StyleColor::ScrollbarGrab as usize] = ImVec4::new(0.20, 0.25, 0.29, 1.00).into();
        colors[imgui::StyleColor::ScrollbarGrabHovered as usize] = ImVec4::new(0.18, 0.22, 0.25, 1.00).into();
        colors[imgui::StyleColor::ScrollbarGrabActive as usize] = ImVec4::new(0.09, 0.21, 0.31, 1.00).into();
        colors[imgui::StyleColor::CheckMark as usize] = ImVec4::new(0.28, 0.56, 1.00, 1.00).into();
        colors[imgui::StyleColor::SliderGrab as usize] = ImVec4::new(0.28, 0.56, 1.00, 1.00).into();
        colors[imgui::StyleColor::SliderGrabActive as usize] = ImVec4::new(0.37, 0.61, 1.00, 1.00).into();
        colors[imgui::StyleColor::Button as usize] = ImVec4::new(0.20, 0.25, 0.29, 1.00).into();
        colors[imgui::StyleColor::ButtonHovered as usize] = ImVec4::new(0.28, 0.56, 1.00, 1.00).into();
        colors[imgui::StyleColor::ButtonActive as usize] = ImVec4::new(0.06, 0.53, 0.98, 1.00).into();
        colors[imgui::StyleColor::Header as usize] = ImVec4::new(0.20, 0.25, 0.29, 0.55).into();
        colors[imgui::StyleColor::HeaderHovered as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.80).into();
        colors[imgui::StyleColor::HeaderActive as usize] = ImVec4::new(0.26, 0.59, 0.98, 1.00).into();
        colors[imgui::StyleColor::Separator as usize] = ImVec4::new(0.20, 0.25, 0.29, 1.00).into();
        colors[imgui::StyleColor::SeparatorHovered as usize] = ImVec4::new(0.10, 0.40, 0.75, 0.78).into();
        colors[imgui::StyleColor::SeparatorActive as usize] = ImVec4::new(0.10, 0.40, 0.75, 1.00).into();
        colors[imgui::StyleColor::ResizeGrip as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.25).into();
        colors[imgui::StyleColor::ResizeGripHovered as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.67).into();
        colors[imgui::StyleColor::ResizeGripActive as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.95).into();
        colors[imgui::StyleColor::Tab as usize] = ImVec4::new(0.11, 0.15, 0.17, 1.00).into();
        colors[imgui::StyleColor::TabHovered as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.80).into();
        colors[imgui::StyleColor::TabActive as usize] = ImVec4::new(0.20, 0.25, 0.29, 1.00).into();
        colors[imgui::StyleColor::TabUnfocused as usize] = ImVec4::new(0.11, 0.15, 0.17, 1.00).into();
        colors[imgui::StyleColor::TabUnfocusedActive as usize] = ImVec4::new(0.11, 0.15, 0.17, 1.00).into();
        colors[imgui::StyleColor::PlotLines as usize] = ImVec4::new(0.61, 0.61, 0.61, 1.00).into();
        colors[imgui::StyleColor::PlotLinesHovered as usize] = ImVec4::new(1.00, 0.43, 0.35, 1.00).into();
        colors[imgui::StyleColor::PlotHistogram as usize] = ImVec4::new(0.90, 0.70, 0.00, 1.00).into();
        colors[imgui::StyleColor::PlotHistogramHovered as usize] = ImVec4::new(1.00, 0.60, 0.00, 1.00).into();
        colors[imgui::StyleColor::TextSelectedBg as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.35).into();
        colors[imgui::StyleColor::DragDropTarget as usize] = ImVec4::new(1.00, 1.00, 0.00, 0.90).into();
        colors[imgui::StyleColor::NavHighlight as usize] = ImVec4::new(0.26, 0.59, 0.98, 1.00).into();
        colors[imgui::StyleColor::NavWindowingHighlight as usize] = ImVec4::new(1.00, 1.00, 1.00, 0.70).into();
        colors[imgui::StyleColor::NavWindowingDimBg as usize] = ImVec4::new(0.80, 0.80, 0.80, 0.20).into();
        colors[imgui::StyleColor::ModalWindowDimBg as usize] = ImVec4::new(0.80, 0.80, 0.80, 0.35).into();
    }
}
