use invisibot_game::{
    game_map::player::Player, persistence::GameId, utils::coordinate::Coordinate,
};

use crate::{
    db_connection::DBConnection,
    postgres_error::PostgresResult,
    repositories::{map_repository, player_repository, round_repository, shot_tile_repository},
};

pub async fn insert_round(
    conn: &DBConnection,
    game_id: GameId,
    round_number: u32,
    players: Vec<Player>,
    shot_tiles: Vec<Coordinate>,
) -> PostgresResult<()> {
    let mut transaction = conn.new_transaction().await?;

    let map = map_repository::get_map_by_game_id(&mut transaction, game_id).await?;
    let round = round_repository::insert(&mut transaction, game_id, round_number, map.id).await?;
    for player in players.into_iter() {
        player_repository::insert(
            &mut transaction,
            player.get_id().clone(),
            game_id.clone(),
            round.round_number as u32,
            player.get_pos().clone(),
            player.get_rotation().clone(),
            player.is_visible(),
        )
        .await?;
    }

    for coord in shot_tiles.into_iter() {
        shot_tile_repository::insert(
            &mut transaction,
            game_id.clone(),
            round.round_number as u32,
            coord,
        )
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}
