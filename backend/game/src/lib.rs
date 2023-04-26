#![forbid(unsafe_code)]

use clients::ClientHandler;
use game_logic::{game::Game, game_config::GameConfig};

pub mod clients;
pub mod game_logic;
pub mod player;
pub mod utils;

pub fn initiate_game<T: ClientHandler>(client_handler: T, config: GameConfig) -> Game<T> {
    Game::new(client_handler, config)
}
