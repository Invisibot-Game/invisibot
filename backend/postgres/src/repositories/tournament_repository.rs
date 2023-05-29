use sqlx::{types::Uuid, Transaction};

use crate::{postgres_error::PostgresResult, tables::tournament::Tournament};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    tournament_name: String,
) -> PostgresResult<Tournament> {
    Ok(sqlx::query_as!(
        Tournament,
        r#"
INSERT INTO tournament (tournament_name)
VALUES           ($1)
RETURNING tournament_id, tournament_name
        "#,
        tournament_name
    )
    .fetch_one(transaction)
    .await?)
}

pub async fn get_all_tournaments(transaction: &mut Transaction<'_, DB>) -> PostgresResult<Vec<Tournament>> {
    Ok(sqlx::query_as!(
        Tournament,
        r#"
SELECT tournament_id, tournament_name
FROM tournament
        "#
    )
    .fetch_all(transaction)
    .await?)
}
