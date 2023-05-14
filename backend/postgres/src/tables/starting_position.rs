use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StartingPosition {
    pub map_id: Uuid,
    pub x: i32,
    pub y: i32,
}
