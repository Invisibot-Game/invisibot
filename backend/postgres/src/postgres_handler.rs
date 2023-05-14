use invisibot_game::{
    async_trait::async_trait,
    game_map::{game_map::GameMap, player::Player},
    persistence::{completed_game::CompletedGame, GameId, PersistenceHandler},
    utils::{coordinate::Coordinate, game_error::GameResult},
};

use crate::{
    db_connection::DBConnection,
    services::{get_game_service, new_game_service, round_service},
};

/// A persistence handler for postgres
pub struct PostgresHandler {
    connection: DBConnection,
}

#[async_trait]
impl PersistenceHandler for PostgresHandler {
    async fn new_game(&self, map: GameMap) -> GameResult<GameId> {
        Ok(new_game_service::insert_new_game(&self.connection, map)
            .await
            .map_err(|e| e.into())?)
    }

    async fn save_round(
        &self,
        game_id: GameId,
        round_number: u32,
        players: Vec<Player>,
        shot_tiles: Vec<Coordinate>,
    ) -> GameResult<()> {
        round_service::insert_round(&self.connection, game_id, round_number, players, shot_tiles)
            .await
            .map_err(|e| e.into())?;
        Ok(())
    }

    async fn game_done(&self, game_id: GameId) -> GameResult<()> {
        // We don't really have anything to do here yet.
        Ok(())
    }

    async fn get_game(&self, game_id: GameId) -> GameResult<CompletedGame> {
        get_game_service::get_finished_game(&self.connection, game_id)
            .await
            .map_err(|e| e.into())
    }
}

impl PostgresHandler {
    /// Create a new PostgresHandler with the provided database connection.
    pub fn new(connection: &DBConnection) -> Self {
        PostgresHandler {
            connection: connection.clone(),
        }
    }
}
