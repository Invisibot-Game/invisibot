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
