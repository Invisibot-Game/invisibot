use invisibot_game::{game_map::game_map::GameMap, persistence::GameId};

use crate::{
    db_connection::DBConnection,
    postgres_error::PostgresResult,
    repositories::{
        game_repository, map_repository, map_wall_repository, starting_position_repository,
    },
};

pub async fn insert_new_game(
    conn: &DBConnection,
    game_map: GameMap,
    num_players: u32,
) -> PostgresResult<GameId> {
    let mut transaction = conn.new_transaction().await?;

    let game = game_repository::insert(&mut transaction, num_players).await?;

    let map =
        map_repository::insert(&mut transaction, game.id, game_map.width, game_map.height).await?;

    for wall_coord in game_map.get_wall_coords().iter() {
        map_wall_repository::insert(&mut transaction, map.id, wall_coord.clone()).await?;
    }

    for starting_pos in game_map.get_starting_positions().iter() {
        starting_position_repository::insert(&mut transaction, map.id, starting_pos.clone())
            .await?;
    }

    transaction.commit().await?;
    Ok(game.id)
}
