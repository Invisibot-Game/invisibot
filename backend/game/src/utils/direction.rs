use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn all_dirs() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}
