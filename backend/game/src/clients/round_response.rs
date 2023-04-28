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
}
