use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A type of tile on the game board.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileType {
    /// An unpassable wall.
    Wall,
    /// An empty tile.
    Empty,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileType::Wall => "#",
                TileType::Empty => ".",
            }
        )
    }
}
