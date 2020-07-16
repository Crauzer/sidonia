use crate::core::{
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

        let game_state = Game::fetch_game_state();
        log::info!("GameState: {:#?}", game_state);

        let render_pipeline = Game::fetch_render_pipeline();
        let renderer = Game::fetch_renderer();
        let scene = Game::fetch_scene();
        let game_clock = Game::fetch_game_clock();
        let hud_manager = Game::fetch_hud_manager();
        let simple_environment_asset = Game::fetch_simple_environment_asset();
        let sun = Game::fetch_sun();
        let world_light_system = Game::fetch_world_light_system();

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
        let new_game_state = Game::fetch_game_state();
        if new_game_state != self.game_state {
            log::info!("Switching to GameState: {:#?}", new_game_state);

            if new_game_state == GameState::GameLoop {}

            self.game_state = new_game_state;
        }

        self.game_state
    }

    fn fetch_game_state() -> GameState {
        unsafe {
            let ptr = memory::convert_file_offset_to_ptr(0x02D788D4) as *mut GameState;

            *ptr
        }
    }
    fn fetch_game_clock() -> *mut RiotGameClock {
        unsafe {
            let game_clock = memory::convert_file_offset_to_ptr(0x02D791A8) as *mut *mut RiotGameClock;

            *game_clock
        }
    }
    fn fetch_render_pipeline() -> *mut RiotRenderPipeline {
        unsafe {
            let render_pipeline = memory::convert_file_offset_to_ptr(0x02D78C68) as *mut *mut RiotRenderPipeline;
            *render_pipeline
        }
    }
    fn fetch_renderer() -> *mut R3dRenderLayer {
        unsafe {
            let render_layer_ptr = memory::convert_file_offset_to_ptr(0x02D79078) as *mut *mut R3dRenderLayer;

            *render_layer_ptr
        }
    }
    fn fetch_scene() -> *mut R3dSceneLayer {
        unsafe {
            let ptr = memory::convert_file_offset_to_ptr(0x02D78F48) as *mut *mut R3dSceneLayer;

            *ptr
        }
    }
    fn fetch_hud_manager() -> *mut RiotHudManager {
        unsafe {
            let ptr = memory::convert_file_offset_to_ptr(0x02D78D24) as *mut *mut RiotHudManager;

            *ptr
        }
    }
    fn fetch_player() -> *mut RiotAiHero {
        unsafe {
            let ptr = memory::convert_file_offset_to_ptr(0x02D77F48) as *mut *mut RiotAiHero;

            *ptr
        }
    }
    fn fetch_simple_environment_asset() -> *mut RiotSimpleEnvironmentAsset {
        unsafe {
            let ptr = memory::convert_file_offset_to_ptr(0x02D78C2C) as *mut *mut RiotSimpleEnvironmentAsset;

            *ptr
        }
    }
    fn fetch_sun() -> *mut R3dSun {
        unsafe {
            let ptr = memory::convert_file_offset_to_ptr(0x02D77E70) as *mut *mut R3dSun;

            *ptr
        }
    }
    fn fetch_world_light_system() -> *mut R3dLightSystem {
        unsafe { memory::convert_file_offset_to_ptr(0x014C7F40) as *mut R3dLightSystem }
    }
}
