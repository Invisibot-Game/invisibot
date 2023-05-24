use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_error::GameError;

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

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "Up",
                Direction::Down => "Down",
                Direction::Right => "Right",
                Direction::Left => "Left",
            }
        )
    }
}

impl TryFrom<String> for Direction {
    type Error = GameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "right" => Direction::Right,
            "left" => Direction::Left,
            v => return Err(GameError::InvalidDirection(v.to_string())),
        })
    }
}

#[cfg(test)]
mod all_dirs_tests {
    use super::Direction;

    #[test]
    fn all_dirs_returns_expected() {
        let all_dirs = Direction::all_dirs();
        assert_eq!(all_dirs[0], Direction::Up);
        assert_eq!(all_dirs[1], Direction::Right);
        assert_eq!(all_dirs[2], Direction::Down);
        assert_eq!(all_dirs[3], Direction::Left);
    }
}

#[cfg(test)]
mod opposite_tests {
    use super::Direction;

    #[test]
    fn opposite_up_is_down() {
        let up = Direction::Up;
        assert_eq!(up.opposite(), Direction::Down)
    }

    #[test]
    fn opposite_down_is_up() {
        let down = Direction::Down;
        assert_eq!(down.opposite(), Direction::Up)
    }

    #[test]
    fn opposite_right_is_left() {
        let right = Direction::Right;
        assert_eq!(right.opposite(), Direction::Left)
    }

    #[test]
    fn opposite_left_is_right() {
        let left = Direction::Left;
        assert_eq!(left.opposite(), Direction::Right)
    }
}

#[cfg(test)]
mod display_test {
    use super::Direction;

    #[test]
    fn display_up_returns_correct() {
        let dir = Direction::Up;
        assert_eq!(dir.to_string(), "Up")
    }

    #[test]
    fn display_down_returns_correct() {
        let dir = Direction::Down;
        assert_eq!(dir.to_string(), "Down")
    }

    #[test]
    fn display_right_returns_correct() {
        let dir = Direction::Right;
        assert_eq!(dir.to_string(), "Right")
    }

    #[test]
    fn display_left_returns_correct() {
        let dir = Direction::Left;
        assert_eq!(dir.to_string(), "Left")
    }
}
