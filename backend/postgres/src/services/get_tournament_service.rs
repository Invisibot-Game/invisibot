use crate::{
    db_connection::DBConnection,
    postgres_error::PostgresResult,
    repositories::{
        tournament_repository
    },
    tables::{tournament::Tournament},
};


// pub async fn get_game(conn: &DBConnection, game_id: GameId) -> PostgresResult<Option<Game>> {
//     let mut transaction = conn.new_transaction().await?;
//     let game = t::try_get_game_by_id(&mut transaction, game_id).await?;
//     transaction.commit().await?;
//     Ok(game)
// }

pub async fn get_tournaments(conn: &DBConnection) -> PostgresResult<Vec<Tournament>> {
    let mut transaction = conn.new_transaction().await?;
    let tournaments = tournament_repository::get_all_tournaments(&mut transaction).await?;
    transaction.commit().await?;
    Ok(tournaments)
}