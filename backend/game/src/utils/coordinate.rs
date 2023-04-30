use serde::{Deserialize, Serialize};

use super::direction::Direction;

/// A coordinate on the game board.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinate {
    /// The x coordinate, 0 is the leftmost column on the game board and values increase rightwards.
    pub x: u32,
    /// The y coordinate, 0 is for the top row on the game board and values increase downwards.
    pub y: u32,
}

/// A shorthand for creating a new coordinate.
#[macro_export]
macro_rules! coord {
    ( $x:expr, $y:expr ) => {{
        Coordinate { x: $x, y: $y }
    }};
}

impl Coordinate {
    /// Get the coordinate one step in the provided direction.
    /// NOT guaranteed to be within game board bounds!
    pub fn translate(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => coord!(self.x, self.y - 1),
            Direction::Down => coord!(self.x, self.y + 1),
            Direction::Right => coord!(self.x + 1, self.y),
            Direction::Left => coord!(self.x - 1, self.y),
        }
    }
}
