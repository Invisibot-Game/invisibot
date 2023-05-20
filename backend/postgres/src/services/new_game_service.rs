use invisibot_game::persistence::GameId;

use crate::{
    db_connection::DBConnection, postgres_error::PostgresResult, repositories::game_repository,
};

pub async fn insert_new_game(
    conn: &DBConnection,
    num_players: u32,
    num_rounds: u32,
    map_dir: String,
) -> PostgresResult<GameId> {
    let mut transaction = conn.new_transaction().await?;

    let game = game_repository::insert(&mut transaction, num_players, num_rounds, map_dir).await?;

    transaction.commit().await?;
    Ok(game.id)
}
