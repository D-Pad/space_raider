// Local imports
pub mod graphics;
pub mod rendering;

use graphics::settings::GraphicSettings;

// Third party imports
use raylib::prelude::*;


pub struct GameEngine {
    pub rl_handle: RaylibHandle,
    pub rl_thread: RaylibThread,
}

impl GameEngine {
    
    pub fn new(window_title: &str, graphics: GraphicSettings) -> Self {

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


