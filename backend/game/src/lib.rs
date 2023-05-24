#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Invisibot game lib
//!
//! Contains the game logic for the invisibot, bot arena game where bots that are invisible fight other bots that are also invisible.
//!

pub use async_trait;

/// Types & logic intended for communication with clients.
pub mod client_handler;
#[doc = "inline"]
pub mod game;
#[doc = "inline"]
pub mod game_config;
#[doc = "inline"]
pub mod game_map;
/// Types & logic intended for persisting games.
pub mod persistence;

mod game_logic;
