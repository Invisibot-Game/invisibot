#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Invisibot game lib
//!
//! Contains the game logic for the invisibot, bot arena game where bots that are invisible fight other bots that are also invisible.
//!

pub use async_trait;
use persistence::GameId;
use serde::{Deserialize, Serialize};

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

/// A response expected to be sent when receiving a ClientHello message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectResponse {
    /// The id of the game you wish to join.
    pub game_id: GameId,
}
