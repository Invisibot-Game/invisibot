use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Player {
    pub id: i32,
    pub game_id: Uuid,
    pub round_number: i32,
    pub x: i32,
    pub y: i32,
    pub rotation: String,
    pub visible: bool,
}
