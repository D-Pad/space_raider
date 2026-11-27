use crate::game_state::GameState;
use engine::graphics::colors::Color;


pub enum GameMode {
    MainMenu,
    GamePlay
}


pub fn draw_scene(mut game_state: GameState) {

    let thread = &game_state.engine.rl_thread;
    let mut draw_thread = game_state.engine.rl_handle.begin_drawing(thread);

    draw_thread.clear_background(Color::BLACK.to_rgb());

    match game_state.mode {

        GameMode::MainMenu => {
            draw_thread.draw_text(game_state.meta.title, 12, 12, 20, Color::WHITE);
        },
        GameMode::GamePlay => {}

    }

}


