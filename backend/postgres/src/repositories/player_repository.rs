use invisibot_game::{
    clients::player_id::PlayerId,
    persistence::GameId,
    utils::{coordinate::Coordinate, direction::Direction},
};
use sqlx::Transaction;

use crate::{postgres_error::PostgresResult, tables::player::Player};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    id: PlayerId,
    game_id: GameId,
    round_number: u32,
    coordinate: Coordinate,
    rotation: Direction,
    visible: bool,
) -> PostgresResult<Player> {
    Ok(sqlx::query_as!(
        Player,
        r#"
INSERT INTO player (id, game_id, round_number, x,  y,  rotation, visible)
VALUES             ($1, $2,      $3,           $4, $5, $6,       $7)
RETURNING id, game_id, round_number, x, y, rotation, visible
        "#,
        id as i32,
        game_id,
        round_number as i32,
        coordinate.x as i32,
        coordinate.y as i32,
        rotation.to_string().to_lowercase(),
        visible,
    )
    .fetch_one(transaction)
    .await?)
}

pub async fn get_players_by_game_and_round(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
    round_number: i32,
) -> PostgresResult<Vec<Player>> {
    Ok(sqlx::query_as!(
        Player,
        r#"
SELECT id, game_id, round_number, x, y, rotation, visible
FROM player
WHERE game_id = $1 AND round_number = $2
        "#,
        game_id,
        round_number
    )
    .fetch_all(transaction)
    .await?)
}
