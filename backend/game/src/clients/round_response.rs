use serde::{Deserialize, Serialize};

use crate::utils::direction::Direction;

/// A clients response when asked to make a move.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResponse {
    dir: Direction,
}

impl RoundResponse {
    /// Create a new `RoundResponse` containing the provided `dir`.
    pub fn new(dir: Direction) -> Self {
        Self { dir }
    }

    /// Get the contained direction.
    pub fn get_dir(&self) -> Direction {
        return self.dir.clone();
    }
}
