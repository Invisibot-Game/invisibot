use serde::{Deserialize, Serialize};

/// A Tournament
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    /// The id of the tournament.
    pub id: uuid::Uuid,
    /// The title of the tournament.
    pub name: String,
}