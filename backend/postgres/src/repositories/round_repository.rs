use invisibot_game::persistence::GameId;
use sqlx::{types::Uuid, Transaction};

use crate::{postgres_error::PostgresResult, tables::round::Round};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
    round_number: u32,
    map_id: Uuid,
) -> PostgresResult<Round> {
    Ok(sqlx::query_as!(
        Round,
        r#"
INSERT INTO round(game_id, round_number, map_id)
VALUES           ($1,      $2,           $3)
RETURNING game_id, round_number, map_id
        "#,
        game_id,
        round_number as i32,
        map_id,
    )
    .fetch_one(transaction)
    .await?)
}

pub async fn get_rounds_by_game_id(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
) -> PostgresResult<Vec<Round>> {
    Ok(sqlx::query_as!(
        Round,
        r#"
SELECT game_id, round_number, map_id
FROM round
WHERE game_id = $1
        "#,
        game_id,
    )
    .fetch_all(transaction)
    .await?)
}
