

mod state_mgmt;
use state_mgmt::*;


// Constant values for game meta data
const GAME_TITLE: &str = "Space Raider";



// ------------------------- Main game loop -------------------------------- //
fn main() {

    let state = game_state::GameState::new(GAME_TITLE);
    state.run();

}


