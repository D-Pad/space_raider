use raylib::prelude::*;
use config::GraphicSettings;


struct GameState {
    title: String,
    graphic_settings: GraphicSettings, 
}

impl GameState {
    
    fn init() -> Self {

        let graphic_settings = GraphicSettings::default();
        Self {
            title: String::from("Space Raider"), 
            graphic_settings 
        }

    }

}


struct GameEngine<'a> {
    rl_handle: RaylibHandle,
    rl_thread: RaylibThread,
    game_state: &'a GameState
}

impl<'a> GameEngine<'a> {
    
    fn new(game_state: &'a GameState) -> Self {

        let (mut rl_handle, rl_thread) = raylib::init()
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


fn main() {

    let game_state = GameState::init(); 
    let mut game_engine = GameEngine::new(&game_state);

    while !game_engine.rl_handle.window_should_close() {
        
        let mut d = game_engine.rl_handle.begin_drawing(&game_engine.rl_thread);
        d.clear_background(Color::BLACK);
        d.draw_text("Space Raider", 12, 12, 20, Color::WHITE);
    
    }

}

