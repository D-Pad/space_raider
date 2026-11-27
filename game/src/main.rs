use engine::{
    GameEngine, 
    graphics::{
        GraphicSettings, 
        colors::Color
    }
}; 


// Constant values for game meta data
const GAME_TITLE: &str = "Space Raider";


struct MetaData {
    title: &'static str
}

impl MetaData {
    
    fn new(title: &'static str) -> Self {
        Self {
            title
        } 
    }

}


// ------------------------- Main game loop -------------------------------- //
fn main() {
   
    // Initialize app settings and game engine 
    let meta_data = MetaData::new(GAME_TITLE);
    let graphic_settings = GraphicSettings::default();
    let mut engine = GameEngine::new(meta_data.title, graphic_settings);
    let t = &engine.rl_thread;

    while !engine.rl_handle.window_should_close() {
       
        let mut d = engine.rl_handle.begin_drawing(t);

        // d.clear_background(Color::BLACK);
        // d.draw_text(meta_data.title, 12, 12, 20, Color::WHITE);
    
    }

}


