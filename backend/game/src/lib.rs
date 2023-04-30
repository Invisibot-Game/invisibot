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

/// Types & logic intended for communication with clients.
pub mod clients;
#[doc = "inline"]
pub mod game;
#[doc = "inline"]
pub mod game_config;
/// Basic utility types
pub mod utils;

mod game_logic;

/// Play a game with clients provided by the client handler and the provided config as game config
pub fn initiate_game<T: ClientHandler>(client_handler: T, config: GameConfig) -> Game<T> {
    Game::new(client_handler, config)
}
