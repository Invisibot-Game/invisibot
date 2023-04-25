#![forbid(unsafe_code)]

use game_logic::game_state::GameState;

pub mod clients;
pub mod game_logic;
pub mod player;
pub mod utils;

pub fn initiate_game() -> GameState {
    GameState::new()
}
