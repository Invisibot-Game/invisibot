use sqlx::Transaction;

use crate::{postgres_error::PostgresResult, tables::game::Game};

use super::DB;

pub async fn insert(transaction: &mut Transaction<'_, DB>) -> PostgresResult<Game> {
    Ok(sqlx::query_as!(
        Game,
        r#"
INSERT INTO game 
DEFAULT VALUES
RETURNING id, created_at
        "#
    )
    .fetch_one(transaction)
    .await?)
}
