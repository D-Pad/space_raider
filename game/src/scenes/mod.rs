use crate::game_state::GameState;


pub enum GameMode {
    MainMenu,
    GamePlay
}


pub fn draw_scene(game_state: GameState) {

    match game_state.mode {

        GameMode::MainMenu => {},
        GameMode::GamePlay => {}

    }

}


