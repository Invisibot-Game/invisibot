use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Map {
    pub id: Uuid,
    pub game_id: Uuid,
    pub width: i32,
    pub height: i32,
}
