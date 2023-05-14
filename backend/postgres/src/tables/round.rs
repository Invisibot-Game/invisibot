use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Round {
    pub game_id: Uuid,
    pub round_number: i32,
    pub map_id: Uuid,
}
