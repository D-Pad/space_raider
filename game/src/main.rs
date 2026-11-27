

mod state_mgmt;
mod scenes;
use state_mgmt::*;


// Constant values for game meta data
const GAME_TITLE: &str = "Space Raider";



// ------------------------- Main game loop -------------------------------- //
fn main() {

    let game = game_state::GameState::new(GAME_TITLE);
    game.run();

}


