use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ShotTile {
    pub game_id: Uuid,
    pub round_number: i32,

    pub x: i32,
    pub y: i32,
}
