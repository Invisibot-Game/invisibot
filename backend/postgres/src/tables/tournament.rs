use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow,Default,sqlx::Type)]
pub struct Tournament {
    pub tournament_id: Uuid,
    pub name: String,
}