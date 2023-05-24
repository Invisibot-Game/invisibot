use std::fmt::Display;

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

    /// Returns the direction necessary for going from `from` to `to`.
    /// Returns None if `from` == `to` or if the movement was purely diagonal.
    pub fn dir_between(from: &Coordinate, to: &Coordinate) -> Option<Direction> {
        let delta_x = (from.x as i32) - (to.x as i32);
        let delta_y = (from.y as i32) - (to.y as i32);
        let delta_x_abs = delta_x.abs();
        let delta_y_abs = delta_y.abs();

        Some(match (delta_x, delta_y) {
            (0, 0) => return None,
            (_, y) if delta_y_abs > delta_x_abs && y < 0 => Direction::Down,
            (_, y) if delta_y_abs > delta_x_abs && y > 0 => Direction::Up,
            (x, _) if delta_x_abs > delta_y_abs && x < 0 => Direction::Right,
            (x, _) if delta_x_abs > delta_y_abs && x > 0 => Direction::Left,
            (_, _) => return None,
        })
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod display_tests {
    use crate::coordinate::Coordinate;

    #[test]
    fn coordinate_displays_correctly() {
        let coord = coord!(6, 3);
        assert_eq!(coord.to_string(), String::from("(6,3)"))
    }
}

#[cfg(test)]
mod translate_tests {
    use crate::direction::Direction;

    #[test]
    fn going_right_returns_right_tile_neighbour() {
        let coord = coord!(4, 4);
        assert_eq!(coord.translate(&Direction::Right), coord!(5, 4))
    }

    #[test]
    fn going_down_returns_below_tile_neighbour() {
        let coord = coord!(4, 4);
        assert_eq!(coord.translate(&Direction::Down), coord!(4, 5))
    }

    #[test]
    fn going_left_returns_left_tile_neighbour() {
        let coord = coord!(4, 4);
        assert_eq!(coord.translate(&Direction::Left), coord!(3, 4))
    }

    #[test]
    fn going_up_returns_above_tile_neighbour() {
        let coord = coord!(4, 4);
        assert_eq!(coord.translate(&Direction::Up), coord!(4, 3))
    }
}

#[cfg(test)]
mod dir_between_tests {
    use crate::{coordinate::Coordinate, direction::Direction};

    #[test]
    fn going_one_step_down_returns_down() {
        assert_eq!(
            Coordinate::dir_between(&coord!(1, 1), &coord!(1, 2)),
            Some(Direction::Down)
        );
    }

    #[test]
    fn going_one_step_right_returns_right() {
        assert_eq!(
            Coordinate::dir_between(&coord!(1, 1), &coord!(2, 1)),
            Some(Direction::Right)
        );
    }

    #[test]
    fn going_one_step_up_returns_up() {
        assert_eq!(
            Coordinate::dir_between(&coord!(1, 1), &coord!(1, 0)),
            Some(Direction::Up)
        );
    }

    #[test]
    fn going_one_step_left_returns_left() {
        assert_eq!(
            Coordinate::dir_between(&coord!(1, 1), &coord!(0, 1)),
            Some(Direction::Left)
        );
    }

    #[test]
    fn more_up_than_right_returns_up() {
        assert_eq!(
            Coordinate::dir_between(&coord!(4, 4), &coord!(2, 0)),
            Some(Direction::Up)
        )
    }

    #[test]
    fn more_up_than_left_returns_up() {
        assert_eq!(
            Coordinate::dir_between(&coord!(4, 4), &coord!(6, 0)),
            Some(Direction::Up)
        )
    }

    #[test]
    fn more_down_than_right_returns_down() {
        assert_eq!(
            Coordinate::dir_between(&coord!(4, 4), &coord!(2, 8)),
            Some(Direction::Down)
        )
    }

    #[test]
    fn more_down_than_left_returns_down() {
        assert_eq!(
            Coordinate::dir_between(&coord!(4, 4), &coord!(6, 8)),
            Some(Direction::Down)
        )
    }

    #[test]
    fn more_right_than_up_returns_right() {
        assert_eq!(
            Coordinate::dir_between(&coord!(4, 4), &coord!(8, 2)),
            Some(Direction::Right)
        )
    }

    #[test]
    fn more_right_than_down_returns_right() {
        assert_eq!(
            Coordinate::dir_between(&coord!(4, 4), &coord!(8, 6)),
            Some(Direction::Right)
        )
    }

    #[test]
    fn more_left_than_up_returns_left() {
        assert_eq!(
            Coordinate::dir_between(&coord!(4, 4), &coord!(0, 2)),
            Some(Direction::Left)
        )
    }

    #[test]
    fn more_left_than_down_returns_left() {
        assert_eq!(
            Coordinate::dir_between(&coord!(4, 4), &coord!(0, 6)),
            Some(Direction::Left)
        )
    }

    #[test]
    fn equal_coords_returns_none() {
        assert_eq!(Coordinate::dir_between(&coord!(4, 4), &coord!(4, 4)), None)
    }

    #[test]
    fn diagonal_up_right_returns_none() {
        assert_eq!(Coordinate::dir_between(&coord!(4, 4), &coord!(6, 2)), None)
    }

    #[test]
    fn diagonal_down_right_returns_none() {
        assert_eq!(Coordinate::dir_between(&coord!(4, 4), &coord!(6, 6)), None)
    }

    #[test]
    fn diagonal_up_left_returns_none() {
        assert_eq!(Coordinate::dir_between(&coord!(4, 4), &coord!(2, 2)), None)
    }

    #[test]
    fn diagonal_down_left_returns_none() {
        assert_eq!(Coordinate::dir_between(&coord!(4, 4), &coord!(2, 6)), None)
    }
}
