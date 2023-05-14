#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Invisibot game lib
//!
//! Contains the game logic for the invisibot, bot arena game where bots that are invisible fight other bots that are also invisible.
//!

use clients::ClientHandler;
use game::Game;
use game_config::GameConfig;
use persistence::PersistenceHandler;
use utils::game_error::GameResult;

pub use async_trait;

/// Types & logic intended for communication with clients.
pub mod clients;
#[doc = "inline"]
pub mod game;
#[doc = "inline"]
pub mod game_config;
#[doc = "inline"]
pub mod game_map;
/// Types & logic intended for persisting games.
pub mod persistence;
/// Basic utility types
pub mod utils;

mod game_logic;

/// Play a game with clients provided by the client handler and the provided config as game config
pub async fn initiate_game<C: ClientHandler, P: PersistenceHandler>(
    client_handler: C,
    persistence_handler: P,
    config: GameConfig,
) -> GameResult<Game<C, P>> {
    Ok(Game::new(client_handler, persistence_handler, config).await?)
}
