#![forbid(unsafe_code)]

use game_logic::{game_state::GameState};

pub mod utils;
pub mod game_logic;
pub mod player;

pub fn initiate_game() -> GameState {
    GameState::new()
}
