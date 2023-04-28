use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResponse {
    round_move: String,
}

impl RoundResponse {
    pub fn new(round: String) -> Self {
        Self { round_move: round }
    }
}
