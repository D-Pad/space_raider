use engine::{
    GameEngine, 
    graphics::{
        settings::GraphicSettings,
        colors::Color
    }
}; 


struct MetaData {
    title: &'static str,
}

impl MetaData {
    
    fn new(title: &'static str) -> Self {
        Self {
            title
        } 
    }

}


pub struct GameState {
    // Add game state management data here as needed. Scene information,
    // player state, etc.
    engine: GameEngine,
}

impl GameState {

    pub fn new(title: &'static str) -> Self {

        // Initialize app settings and game engine 
        let meta = MetaData::new(title);
        let graphic_settings = GraphicSettings::new(810, 1080);
        let engine = GameEngine::new(meta.title, graphic_settings);

        Self {
            engine
        }

    }

    pub fn run(mut self) {

        let thread = &self.engine.rl_thread;

        while !self.engine.rl_handle.window_should_close() {
           
            let mut d = self.engine.rl_handle.begin_drawing(thread);

            // d.clear_background(Color::BLACK);
            // d.draw_text(meta_data.title, 12, 12, 20, Color::WHITE);
        
        }

    }

}


