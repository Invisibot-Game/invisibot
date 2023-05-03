use serde::{Deserialize, Serialize};

use crate::utils::direction::Direction;

/// A clients response when asked to make a move.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoundResponse {
    /// A request to move in the provided direction.
    Move(Direction),
    /// A request to rotate in place to face the provided direction.
    Rotate(Direction),
    /// A request to shoot 1 shot in the currently facing direction.
    Shoot,
}

impl RoundResponse {
    /// Create a new `RoundResponse` to move in the provided direction.
    pub fn move_in_dir(dir: Direction) -> Self {
        Self::Move(dir)
    }

    /// Create a new `RoundResponse` to rotate in place to face the provided direction.
    pub fn rotate(dir: Direction) -> Self {
        Self::Rotate(dir)
    }

    /// Shoot a shot in the way you're currently facing.
    pub fn shoot() -> Self {
        Self::Shoot
    }
}
