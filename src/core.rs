use crate::{
    core::{
        d3d9::direct3d9::IDirect3D9,
        game::{Game, GameState},
        riot::{r3d::render_layer::R3dRenderLayer, x3d::d3d9::device::X3dD3d9Device},
        ui::Ui,
    },
    CORE,
};
use imgui_dx9_renderer::IDirect3DDevice9;
use std::error::Error;
use winapi::{
    _core::ptr::NonNull,
    ctypes::c_int,
    shared::{d3d9::LPDIRECT3DDEVICE9, minwindef::LPVOID},
};

pub mod d3d9;
pub mod detours;
pub mod errors;
pub mod game;
pub mod msvc;
pub mod riot;
pub mod ui;
pub mod utilities;

pub struct Core {
    game: Game,
    ui: Ui,
}

#[derive(Debug)]
pub enum CoreStatus {
    Idle,
    Running,
    Exit,
}

#[derive(Debug)]
pub enum CoreExitReason {
    GameEnded,
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

        unsafe {
            detours::initialize_riot_x3d_d3d9_device_end_scene_hook(Core::end_scene);
        }

        Core {
            game: Game::new(),
            ui: Ui::new(),
        }
    }

    pub fn ui_mut(&mut self) -> &mut Ui {
        &mut self.ui
    }

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
                        CoreStatus::Idle => {}
                        CoreStatus::Running => {}
                        CoreStatus::Exit => {}
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
