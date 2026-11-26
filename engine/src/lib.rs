pub mod graphics;

use raylib::prelude::*;
use graphics::GraphicSettings;


struct GameEngine {
    rl_handle: RaylibHandle,
    rl_thread: RaylibThread,
}

impl GameEngine {
    
    fn new(window_title: &str, graphics: GraphicSettings) -> Self {

        let (rl_handle, rl_thread) = raylib::init()
            .size( 
                graphics.screen_width, 
                graphics.screen_height)
            .title(window_title)
            .build();
        
        Self {
            rl_handle,
            rl_thread,
        }
    
    }

}


pub fn run(window_title: &str, graphic_settings: GraphicSettings) {
 
    let mut engine = GameEngine::new(window_title, graphic_settings);
    let t = &engine.rl_thread;

    while !engine.rl_handle.window_should_close() {
       
        let mut d = engine.rl_handle.begin_drawing(t);

        d.clear_background(Color::BLACK);
        d.draw_text(window_title, 12, 12, 20, Color::WHITE);
    
    }

}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn it_works() {
    
    }

}


