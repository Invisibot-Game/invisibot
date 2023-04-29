use serde::{Deserialize, Serialize};

use super::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

#[macro_export]
macro_rules! coord {
    ( $x:expr, $y:expr ) => {{
        Coordinate { x: $x, y: $y }
    }};
}

impl Coordinate {
    pub fn translate(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => coord!(self.x, self.y - 1),
            Direction::Down => coord!(self.x, self.y + 1),
            Direction::Right => coord!(self.x + 1, self.y),
            Direction::Left => coord!(self.x - 1, self.y),
        }
    }
}
