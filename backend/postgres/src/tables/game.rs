use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Game {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}
