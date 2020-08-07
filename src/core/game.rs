use crate::core::{
    globals,
    riot::{
        ai_hero::RiotAiHero,
        game_clock::RiotGameClock,
        hud_manager::RiotHudManager,
        r3d::{light_system::R3dLightSystem, render_layer::R3dRenderLayer, scene::R3dSceneLayer, sun::R3dSun},
        render_pipeline::RiotRenderPipeline,
        simple_environment::RiotSimpleEnvironmentAsset,
    },
    utilities::memory,
};

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameState {
    PreGame = 0,
    Spawn = 1,
    GameLoop = 2,
    EndGame = 3,
    PreExit = 4,
}

#[derive(Copy, Clone)]
pub struct Game {
    game_state: GameState,
    game_clock: *mut RiotGameClock,
    render_pipeline: *mut RiotRenderPipeline,
    renderer: *mut R3dRenderLayer,
    scene: *mut R3dSceneLayer,
    hud_manager: *mut RiotHudManager,
    simple_environment_asset: *mut RiotSimpleEnvironmentAsset,
    sun: *mut R3dSun,
    world_light_system: *mut R3dLightSystem,
}

impl Game {
    pub fn new() -> Self {
        log::info!("Initializing Game...");

        unsafe {
            let game_state = *globals::fetch_global(globals::GAME_STATE);
            log::info!("GameState: {:#?}", game_state);

            let render_pipeline = globals::fetch_global_ptr(globals::RENDER_PIPELINE);
            let renderer = globals::fetch_global_ptr(globals::RENDERER);
            let scene = globals::fetch_global_ptr(globals::SCENE);
            let game_clock = globals::fetch_global_ptr(globals::GAME_CLOCK);
            let hud_manager = globals::fetch_global_ptr(globals::HUD_MANAGER);
            let simple_environment_asset = globals::fetch_global_ptr(globals::SIMPLE_ENVIRONMENT_ASSET);
            let sun = globals::fetch_global_ptr(globals::SUN);
            let world_light_system = globals::fetch_global(globals::WORLD_LIGHT_SYSTEM);

            Game {
                game_state,
                game_clock,
                render_pipeline,
                renderer,
                scene,
                hud_manager,
                simple_environment_asset,
                sun,
                world_light_system,
            }
        }
    }

    pub fn game_clock(&self) -> Option<&RiotGameClock> {
        unsafe { self.game_clock.as_ref() }
    }
    pub fn game_clock_mut(&mut self) -> Option<&mut RiotGameClock> {
        unsafe { self.game_clock.as_mut() }
    }
    pub fn renderer(&self) -> Option<&R3dRenderLayer> {
        unsafe { self.renderer.as_ref() }
    }
    pub fn renderer_mut(&mut self) -> Option<&mut R3dRenderLayer> {
        unsafe { self.renderer.as_mut() }
    }
    pub fn scene(&self) -> Option<&R3dSceneLayer> {
        unsafe { self.scene.as_ref() }
    }
    pub fn scene_mut(&mut self) -> Option<&mut R3dSceneLayer> {
        unsafe { self.scene.as_mut() }
    }
    pub fn hud_manager(&self) -> Option<&RiotHudManager> {
        unsafe { self.hud_manager.as_ref() }
    }
    pub fn hud_manager_mut(&mut self) -> Option<&mut RiotHudManager> {
        unsafe { self.hud_manager.as_mut() }
    }
    pub fn simple_environment_asset(&self) -> Option<&RiotSimpleEnvironmentAsset> {
        unsafe { self.simple_environment_asset.as_ref() }
    }
    pub fn simple_environment_asset_mut(&mut self) -> Option<&mut RiotSimpleEnvironmentAsset> {
        unsafe { self.simple_environment_asset.as_mut() }
    }
    pub fn sun(&self) -> Option<&R3dSun> {
        unsafe { self.sun.as_ref() }
    }
    pub fn sun_mut(&mut self) -> Option<&mut R3dSun> {
        unsafe { self.sun.as_mut() }
    }
    pub fn world_light_system(&self) -> Option<&R3dLightSystem> {
        unsafe { self.world_light_system.as_ref() }
    }
    pub fn world_light_system_mut(&mut self) -> Option<&mut R3dLightSystem> {
        unsafe { self.world_light_system.as_mut() }
    }

    pub fn is_renderer_initialized(&self) -> bool {
        unsafe {
            if let Some(renderer) = self.renderer.as_ref() {
                renderer.is_initialized()
            } else {
                false
            }
        }
    }

    pub fn update(&mut self) -> GameState {
        // Update Game State
        let new_game_state = unsafe { *globals::fetch_global(globals::GAME_STATE) };
        if new_game_state != self.game_state {
            log::info!("Switching to GameState: {:#?}", new_game_state);

            if new_game_state == GameState::GameLoop {}

            self.game_state = new_game_state;
        }

        self.game_state
    }
}
