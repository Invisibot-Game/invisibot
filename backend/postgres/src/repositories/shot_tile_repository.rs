use invisibot_game::{persistence::GameId, utils::coordinate::Coordinate};
use sqlx::Transaction;

use crate::{postgres_error::PostgresResult, tables::shot_tile::ShotTile};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
    round_number: u32,
    coordinate: Coordinate,
) -> PostgresResult<ShotTile> {
    Ok(sqlx::query_as!(
        ShotTile,
        r#"
INSERT INTO shot_tile (game_id, round_number, x,  y)
VALUES                ($1,      $2,           $3, $4)
RETURNING game_id, round_number, x, y
        "#,
        game_id,
        round_number as i32,
        coordinate.x as i32,
        coordinate.y as i32,
    )
    .fetch_one(transaction)
    .await?)
}

pub async fn get_by_game_and_round(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
    round_number: i32,
) -> PostgresResult<Vec<ShotTile>> {
    Ok(sqlx::query_as!(
        ShotTile,
        r#"
SELECT game_id, round_number, x, y
FROM shot_tile
WHERE game_id = $1 AND round_number = $2
        "#,
        game_id,
        round_number
    )
    .fetch_all(transaction)
    .await?)
}
