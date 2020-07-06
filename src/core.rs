use crate::{
    core::{
        game::{Game, GameState},
        riot::x3d::d3d9::device::X3dD3d9Device,
        ui::Ui,
    },
    CORE,
};
use winapi::shared::{d3d9::IDirect3DDevice9, d3d9types::D3DPRESENT_PARAMETERS};

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
            detours::initialize_riot_x3d_d3d9_device_end_scene_hook(Core::end_scene).expect("Failed to initialize EndScene hook");
            detours::initialize_riot_x3d_d3d9_device_reset_hook(Core::reset_device).expect("Failed to initialize ResetDevice hook");
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
        self.ui.render();
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
    fn reset_device(device: *mut IDirect3DDevice9, present_parameters: *mut D3DPRESENT_PARAMETERS) {
        log::info!("Resetting device...");

        unsafe {
            match (device.as_mut(), present_parameters.as_mut()) {
                (Some(device), Some(present_parameters)) => {
                    // Grab CORE
                    if let Some(core) = CORE.as_mut() {
                        // Grab a MutexGuard to CORE
                        let mut core = core.lock().unwrap();

                        // Invalidate UI renderer here so it can get updated
                        core.ui_mut().reset();
                    }
                }
                _ => {
                    // Panic here because we can't render UI anymore
                    panic!("ResetDevice hook received either a null IDirect3DDevice9 or null D3DPRESENT_PARAMETERS");
                }
            }

            detours::RiotX3dD3d9DeviceResetHook.call(device, present_parameters);
        }
    }
}
