use invisibot_common::GameId;

use crate::{
    db_connection::DBConnection, postgres_error::PostgresResult, repositories::tournament_repository,
    tables::tournament::Tournament,
};

pub async fn insert_new_tournament(
    conn: &DBConnection,
    name: String
) -> PostgresResult<Tournament> {
    let mut transaction = conn.new_transaction().await?;

    let tournament = tournament_repository::insert(&mut transaction, name).await?;

    transaction.commit().await?;
    Ok(tournament)
}
