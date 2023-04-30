#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]

//! # Invisibot game lib
//!
//! Contains the game logic for the invisibot, bot arena game where bots that are invisible fight other bots that are also invisible.
//!

use clients::ClientHandler;
use game::Game;
use game_config::GameConfig;

pub mod clients;
pub mod game;
pub mod game_config;
pub mod utils;

mod game_logic;
pub fn initiate_game<T: ClientHandler>(client_handler: T, config: GameConfig) -> Game<T> {
    Game::new(client_handler, config)
}
