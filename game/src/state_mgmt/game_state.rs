use engine::{
    GameEngine, 
    graphics::{
        settings::GraphicSettings,
    }
}; 

use crate::scenes::GameMode;


// ---------------------------- META DATA STRUCT --------------------------- //
struct MetaData {
    pub title: &'static str,
}

impl MetaData {
    
    fn new(title: &'static str) -> Self {
        Self {
            title
        } 
    }
}


// --------------------------- GAME STATE STRUCT --------------------------- //
pub struct GameState {
    // Add game state management data here as needed. Scene information,
    // player state, etc.
    pub engine: GameEngine,
    pub mode: GameMode,
    pub meta: MetaData,
}

impl GameState {

    pub fn new(title: &'static str) -> Self {

        // Initialize app settings and game engine 
        let meta = MetaData::new(title);
        let graphic_settings = GraphicSettings::new(810, 1080);
        let engine = GameEngine::new(meta.title, graphic_settings);
        let mode = GameMode::MainMenu;

        Self {
            engine,
            mode,
            meta
        }

    }

    pub fn run(mut self) {

        while !self.engine.rl_handle.window_should_close() {
        
        }

    }

}


