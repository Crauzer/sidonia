use crate::core::game::{Game, GameState};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::LPVOID;
use winapi::_core::ptr::NonNull;
use crate::core::riot::r3d::render_layer::R3dRenderLayer;
use crate::core::d3d9::direct3d9::IDirect3D9;
use crate::core::ui::Ui;
use std::error::Error;
use crate::CORE;
use winapi::shared::d3d9::LPDIRECT3DDEVICE9;
use imgui_dx9_renderer::IDirect3DDevice9;
use crate::core::riot::x3d::d3d9::device::X3dD3d9Device;

pub mod errors;
pub mod game;
pub mod utilities;
pub mod msvc;
pub mod riot;
pub mod detours;
pub mod d3d9;
pub mod ui;

pub struct Core {
    game: Game,
    ui: Ui,
}

#[derive(Debug)]
pub enum CoreStatus {
    Idle,
    Running,
    Exit
}

#[derive(Debug)]
pub enum CoreExitReason {
    GameEnded
}

impl From<GameState> for CoreStatus {
    fn from(game_state: GameState) -> Self {
        match game_state {
            GameState::PreGame => CoreStatus::Idle,
            GameState::Spawn => CoreStatus::Idle,
            GameState::GameLoop => CoreStatus::Running,
            GameState::EndGame => CoreStatus::Idle,
            GameState::PreExit => CoreStatus::Exit,
        }
    }
}

impl Core {
    pub fn initialize() -> Self {
        log::info!("Initializing core...");

        //unsafe { detours::initialize_init_renderer_hook(Core::init_renderer).expect("Failed to hook InitRenderer"); }

        unsafe { detours::initialize_riot_x3d_d3d9_device_end_scene_hook(Core::end_scene); }

        Core {
            game: Game::new(),
            ui: Ui::new(),
        }
    }

    pub fn ui_mut(&mut self) -> &mut Ui { &mut self.ui }

    pub fn update(&mut self, d3d9_device: &mut X3dD3d9Device) -> CoreStatus {
        let status = CoreStatus::from(self.game.update());

        self.update_ui(d3d9_device);

        status
    }

    fn update_ui(&mut self, d3d9_device: &mut X3dD3d9Device) {
        let game = &mut self.game;
        self.ui.update(game, d3d9_device);
    }

    fn end_scene(device: *mut X3dD3d9Device) {
        unsafe {
            // Grab CORE
            if let Some(core) = CORE.as_mut() {
                // Grab a MutexGuard to CORE
                let mut core = core.lock().unwrap();

                if let Some(device) = device.as_mut() {
                    let status = core.update(device);
                    match status {
                        CoreStatus::Idle => {},
                        CoreStatus::Running => {},
                        CoreStatus::Exit => {

                        }
                    }
                }
            }

            detours::RiotX3dD3D9DeviceEndSceneHook.call(device);
        }
    }

    //fn init_renderer(unk: LPVOID) -> c_int {
    //    log::info!("INITIALIZING RENDERER");
    //    unsafe { detours::InitRendererHook.call(unk) }
    //}
}
