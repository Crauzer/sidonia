use crate::{
    core::{
        game::{Game, GameState},
        riot::{camera_logic::RiotCameraLogicMode, x3d::d3d9::device::X3dD3d9Device},
        ui::Ui,
    },
    CORE,
};
use std::error::Error;
use winapi::{
    _core::intrinsics::transmute,
    shared::{d3d9::IDirect3DDevice9, d3d9types::D3DPRESENT_PARAMETERS},
};
use crate::core::hook_manager::HookManager;

pub mod d3d9;
pub mod detours;
pub mod errors;
pub mod game;
pub mod globals;
pub mod hook_manager;
pub mod msvc;
pub mod riot;
pub mod ui;
pub mod utilities;

pub struct Core {
    hook_manager: HookManager,
    game: Game,
    ui: Ui,
    status: CoreStatus,
    first_ui_update_since_reset: bool,
    should_exit: bool,
}

#[derive(Copy, Clone)]
pub struct CoreFetchData {
    should_reset: bool,
    should_exit: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CoreStatus {
    Idle,
    PreLoad,
    Running,
    Exit,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CoreExitReason {
    GameEnded,
}

impl From<GameState> for CoreStatus {
    fn from(game_state: GameState) -> Self {
        match game_state {
            GameState::PreGame => CoreStatus::PreLoad,
            GameState::Spawn => CoreStatus::PreLoad,
            GameState::GameLoop => CoreStatus::Running,
            GameState::EndGame => CoreStatus::Idle,
            GameState::PreExit => CoreStatus::Exit,
        }
    }
}

impl Core {
    pub fn initialize() -> Self {
        log::info!("Initializing core...");

        Core::initialize_detours();

        Core {
            hook_manager: HookManager::new(),
            game: Game::new(),
            ui: Ui::new(),
            status: CoreStatus::PreLoad,
            first_ui_update_since_reset: true,
            should_exit: false,
        }
    }
    fn initialize_detours() {
        log::info!("Initializing detours...");

        //unsafe { detours::initialize_init_renderer_hook(Core::init_renderer).expect("Failed to hook InitRenderer"); }

        unsafe {
            // TODO: make a hook register function
            detours::initialize_riot_x3d_d3d9_device_end_scene_hook(Core::end_scene).expect("Failed to initialize EndScene hook");
            detours::initialize_riot_x3d_d3d9_device_reset_hook(Core::reset_device).expect("Failed to initialize ResetDevice hook");
        }
    }

    pub fn ui_mut(&mut self) -> &mut Ui {
        &mut self.ui
    }
    pub fn status(&self) -> CoreStatus {
        self.status
    }
    pub fn scheduled_for_exit(&self) -> bool {
        self.should_exit
    }

    pub fn update(&mut self, d3d9_device: &mut X3dD3d9Device) -> CoreStatus {
        if self.status == CoreStatus::Exit {
            return CoreStatus::Exit;
        }

        self.status = if self.should_exit {
            CoreStatus::Exit
        } else {
            CoreStatus::from(self.game.update())
        };

        match self.status {
            CoreStatus::Idle => {}
            CoreStatus::Running => {
                self.update_ui(d3d9_device);
                self.first_ui_update_since_reset = false;
            }
            CoreStatus::Exit => {}
            CoreStatus::PreLoad => {}
        }

        self.status
    }

    fn update_ui(&mut self, d3d9_device: &mut X3dD3d9Device) {
        self.ui.update(&self.game, d3d9_device);
        self.ui.render();
        if !self.first_ui_update_since_reset {
            self.ui.fetch_game_data(&mut self.game);
        }

        let core_fetch_data = self.ui.fetch_core_data();
        if core_fetch_data.should_exit {
            self.signal_detach();
        }
        if core_fetch_data.should_reset {
            self.reset();
        }
    }

    pub fn exit(&mut self) {
        log::info!("Core exiting...");

        match self.disable_hooks() {
            Ok(()) => {}
            Err(err) => {
                log::error!("Failed to disable hooks during exit!");
                log::error!("err: {:#?}", err);
            }
        }

        self.status = CoreStatus::Exit;
    }
    fn disable_hooks(&self) -> Result<(), Box<dyn Error>> {
        log::info!("Disabling hooks...");

        unsafe {
            //detours::disable_hook(&detours::InitRendererHook)?;
            detours::disable_hook(&detours::RiotX3dD3D9DeviceEndSceneHook)?;
            detours::disable_hook(&detours::RiotX3dD3d9DeviceResetHook)?;
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        log::info!("Core resetting...");

        self.ui.reset();

        self.first_ui_update_since_reset = true;
    }

    pub fn signal_detach(&mut self) {
        self.should_exit = true;
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
                        CoreStatus::PreLoad => {}
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
