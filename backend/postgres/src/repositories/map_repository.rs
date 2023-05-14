use invisibot_game::persistence::GameId;
use sqlx::Transaction;

use crate::{postgres_error::PostgresResult, tables::map::Map};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
    width: u32,
    height: u32,
) -> PostgresResult<Map> {
    Ok(sqlx::query_as!(
        Map,
        r#"
INSERT INTO map (game_id, width, height)
VALUES          ($1,      $2,    $3)
RETURNING id, game_id, width, height
        "#,
        game_id,
        width as i32,
        height as i32,
    )
    .fetch_one(transaction)
    .await?)
}

pub async fn get_map_by_game_id(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
) -> PostgresResult<Map> {
    Ok(sqlx::query_as!(
        Map,
        r#"
SELECT id, game_id, width, height
FROM map
WHERE game_id = $1
        "#,
        game_id
    )
    .fetch_one(transaction)
    .await?)
}
