use crate::core::{
    riot::{
        ai_hero::RiotAiHero,
        game_clock::RiotGameClock,
        hud_manager::RiotHudManager,
        r3d::{render_layer::R3dRenderLayer, scene::R3dSceneLayer},
        render_pipeline::RiotRenderPipeline,
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

        Game {
            game_state,
            game_clock,
            render_pipeline,
            renderer,
            scene,
            hud_manager,
        }
    }

    pub fn game_clock(&self) -> Option<&'static RiotGameClock> {
        unsafe { self.game_clock.as_ref::<'static>() }
    }
    pub fn renderer_mut(&mut self) -> Option<&'static mut R3dRenderLayer> {
        unsafe { self.renderer.as_mut::<'static>() }
    }
    pub fn scene_mut(&mut self) -> Option<&'static mut R3dSceneLayer> {
        unsafe { self.scene.as_mut::<'static>() }
    }
    pub fn hud_manager_mut(&mut self) -> Option<&'static mut RiotHudManager> {
        unsafe { self.hud_manager.as_mut::<'static>() }
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
}
