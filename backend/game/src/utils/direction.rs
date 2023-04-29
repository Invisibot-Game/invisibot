use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[macro_export]
macro_rules! all_dirs {
    () => {{
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }};
}
