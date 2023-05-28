use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Contestant {
    pub contestant_id: Uuid,
    pub name: String,
    pub docker_file: String,
    pub tournament_id: Uuid,
}