use invisibot_common::GameId;
use sqlx::Transaction;

use crate::{postgres_error::PostgresResult, tables::game::Game};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    num_players: u32,
    num_rounds: u32,
    map_dir: String,
) -> PostgresResult<Game> {
    Ok(sqlx::query_as!(
        Game,
        r#"
INSERT INTO game (num_players, max_num_rounds, map_dir)
VALUES           ($1,          $2,             $3)
RETURNING id, created_at, started_at, finished_at, num_players, max_num_rounds, map_dir
        "#,
        num_players as i32,
        num_rounds as i32,
        map_dir
    )
    .fetch_one(&mut **transaction)
    .await?)
}

pub async fn get_game_by_id(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
) -> PostgresResult<Game> {
    Ok(sqlx::query_as!(
        Game,
        r#"
SELECT id, created_at, started_at, finished_at, num_players, max_num_rounds, map_dir
FROM game
WHERE id = $1
        "#,
        game_id
    )
    .fetch_one(&mut **transaction)
    .await?)
}

pub async fn try_get_game_by_id(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
) -> PostgresResult<Option<Game>> {
    Ok(sqlx::query_as!(
        Game,
        r#"
SELECT id, created_at, started_at, finished_at, num_players, max_num_rounds, map_dir
FROM game
WHERE id = $1
        "#,
        game_id
    )
    .fetch_optional(&mut **transaction)
    .await?)
}

pub async fn get_all_games(transaction: &mut Transaction<'_, DB>) -> PostgresResult<Vec<Game>> {
    Ok(sqlx::query_as!(
        Game,
        r#"
SELECT id, created_at, started_at, finished_at, num_players, max_num_rounds, map_dir
FROM game
        "#
    )
    .fetch_all(&mut **transaction)
    .await?)
}

pub async fn start_game_with_id(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
) -> PostgresResult<()> {
    sqlx::query_as!(
        Game,
        r#"
UPDATE game
SET started_at = now()
WHERE id = $1
    "#,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

pub async fn end_game_with_id(
    transaction: &mut Transaction<'_, DB>,
    game_id: GameId,
) -> PostgresResult<()> {
    sqlx::query_as!(
        Game,
        r#"
UPDATE game
SET finished_at = now()
WHERE id = $1
    "#,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
