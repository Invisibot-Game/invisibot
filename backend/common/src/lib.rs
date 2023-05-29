#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Invisibot common
//! Common types & methods used by all parts of the common.

use uuid::Uuid;

/// A unique identifier for a game.
pub type GameId = Uuid;

#[doc = "inline"]
pub mod coordinate;
#[doc = "inline"]
pub mod direction;
#[doc = "inline"]
pub mod game_error;
#[doc = "inline"]
pub mod player_id;
#[doc = "inline"]
pub mod tile_type;
#[doc = "inline"]
pub mod tournament;

