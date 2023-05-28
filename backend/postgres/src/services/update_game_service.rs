use invisibot_common::GameId;
use invisibot_game::game_map::map::GameMap;

use crate::{
    db_connection::DBConnection,
    postgres_error::PostgresResult,
    repositories::{
        game_repository, map_repository, map_wall_repository, starting_position_repository,
    },
};

pub async fn set_game_started(conn: &DBConnection, game_id: GameId) -> PostgresResult<()> {
    let mut transaction = conn.new_transaction().await?;

    game_repository::start_game_with_id(&mut transaction, game_id).await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn set_game_finished(conn: &DBConnection, game_id: GameId) -> PostgresResult<()> {
    let mut transaction = conn.new_transaction().await?;

    game_repository::end_game_with_id(&mut transaction, game_id).await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn set_game_map(
    conn: &DBConnection,
    game_id: GameId,
    game_map: GameMap,
) -> PostgresResult<()> {
    let mut transaction = conn.new_transaction().await?;

    let map =
        map_repository::insert(&mut transaction, game_id, game_map.width, game_map.height).await?;

    for wall_coord in game_map.get_wall_coords().iter() {
        map_wall_repository::insert(&mut transaction, map.id, wall_coord.clone()).await?;
    }

    for starting_pos in game_map.get_starting_positions().iter() {
        starting_position_repository::insert(&mut transaction, map.id, starting_pos.clone())
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}
