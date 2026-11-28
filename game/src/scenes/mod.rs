use crate::state_mgmt::game_state::GameState;
use engine::rendering::start_new_scene;


pub enum GameMode {
    MainMenu,
    GamePlay
}


pub fn draw_scene(game_state: &mut GameState) { 

    match game_state.mode {

        GameMode::MainMenu => {
            
            // draw_thread.draw_text(game_state.meta.title, 12, 12, 20, Color::WHITE);
        
        },
        
        GameMode::GamePlay => {}

    };

    // Start drawing a new scene. This function call only clears the background
    // and starts a new frame. More logic is needed to actually draw the scene.
    start_new_scene(&mut game_state.engine);

    // Start drawing objects here

}


