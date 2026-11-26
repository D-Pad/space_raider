use raylib::prelude::*;
pub mod config;
use config::GraphicSettings;


struct GameState {
    title: String,
    graphic_settings: GraphicSettings, 
}

impl GameState {
    
    fn init() -> Self {

        let graphic_settings = GraphicSettings::default();
        Self {
            title: String::from("Cyber Raid"), 
            graphic_settings 
        }

    }

}


struct GameEngine {
    rl_handle: RaylibHandle,
    rl_thread: RaylibThread,
    game_state: GameState
}

impl GameEngine {
    
    fn new() -> Self {

        let game_state = GameState::init();

        let (rl_handle, rl_thread) = raylib::init()
            .size( 
                game_state.graphic_settings.screen_width, 
                game_state.graphic_settings.screen_height)
            .title(&game_state.title)
            .build();
        
        Self {
            rl_handle,
            rl_thread,
            game_state 
        }
    }

}


pub fn run() {
  
    let mut engine = GameEngine::new();

    while !engine.rl_handle.window_should_close() {
       
        let t = &engine.rl_thread;
        let mut d = engine.rl_handle.begin_drawing(t);
        d.clear_background(Color::BLACK);
        d.draw_text(&engine.game_state.title, 12, 12, 20, Color::WHITE);
    
    }

}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn it_works() {
    
    }

}


