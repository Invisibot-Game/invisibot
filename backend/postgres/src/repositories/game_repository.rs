use invisibot_game::persistence::GameId;
use sqlx::Transaction;

use crate::{postgres_error::PostgresResult, tables::game::Game};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    num_players: u32,
) -> PostgresResult<Game> {
    Ok(sqlx::query_as!(
        Game,
        r#"
INSERT INTO game (num_players)
VALUES           ($1)
RETURNING id, created_at, num_players
        "#,
        num_players as i32
    )
    .fetch_one(transaction)
    .await?)
}

pub async fn get_game_by_id(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
) -> PostgresResult<Game> {
    Ok(sqlx::query_as!(
        Game,
        r#"
SELECT id, created_at, num_players
FROM game
WHERE id = $1
        "#,
        game_id
    )
    .fetch_one(transaction)
    .await?)
}
