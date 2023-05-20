use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Game {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub num_players: i32,
    pub max_num_rounds: i32,
    pub map_dir: String,
}

impl Game {
    pub fn has_started(&self) -> bool {
        self.started_at.is_some()
    }
}
