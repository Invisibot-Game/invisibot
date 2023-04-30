use serde::{Deserialize, Serialize};

/// A direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Direction {
    /// The 'Up' direction (lower y values)
    Up,
    /// The 'Down' direction (higher y values)
    Down,
    /// The 'Right' direction (higher x values)
    Right,
    /// The 'Left' direction (lower x values)
    Left,
}

impl Direction {
    /// A list of all available directions.
    pub fn all_dirs() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }

    /// Get the opposite direction of the current one, e.g. Left -> Right.
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}
