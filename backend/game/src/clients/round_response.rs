use serde::{Deserialize, Serialize};

use crate::utils::direction::Direction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResponse {
    dir: Direction,
}

impl RoundResponse {
    pub fn new(dir: Direction) -> Self {
        Self { dir }
    }

    pub fn get_dir(&self) -> Direction {
        return self.dir.clone();
    }
}
