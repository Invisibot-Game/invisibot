use invisibot_game::{
    async_trait::async_trait,
    game_config::GameConfig,
    game_map::{game_map::GameMap, player::Player},
    persistence::{completed_game::CompletedGame, GameId, PersistenceHandler},
    utils::{coordinate::Coordinate, game_error::GameResult},
};
use sqlx::types::chrono::{DateTime, Utc};

use crate::{
    db_connection::DBConnection,
    postgres_error::PostgresResult,
    services::{get_game_service, new_game_service, round_service, update_game_service},
};

/// A persistence handler for postgres
#[derive(Debug, Clone)]
pub struct PostgresHandler {
    connection: DBConnection,
}

#[async_trait]
impl PersistenceHandler for PostgresHandler {
    async fn get_game_config(&self, game_id: GameId) -> GameResult<GameConfig> {
        get_game_service::get_game_config(&self.connection, game_id)
            .await
            .map_err(|e| e.into())
    }

    async fn set_game_map(&self, game_id: GameId, map: GameMap) -> GameResult<()> {
        update_game_service::set_game_map(&self.connection, game_id, map)
            .await
            .map_err(|e| e.into())?;

        Ok(())
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
}

impl PostgresHandler {
    /// Create a new PostgresHandler with the provided database connection.
    pub fn new(connection: &DBConnection) -> Self {
        PostgresHandler {
            connection: connection.clone(),
        }
    }

    /// Start a new game
    pub async fn new_game(
        &self,
        num_players: u32,
        num_rounds: u32,
        map_dir: String,
    ) -> GameResult<GameId> {
        Ok(
            new_game_service::insert_new_game(&self.connection, num_players, num_rounds, map_dir)
                .await
                .map_err(|e| e.into())?,
        )
    }

    /// Mark a game as started.
    pub async fn set_game_started(&self, game_id: GameId) -> PostgresResult<()> {
        update_game_service::set_game_started(&self.connection, game_id).await
    }

    /// Returns the number of players for this game or an error if the game does not exist (or another error occurred).
    pub async fn get_num_players_for_game(&self, game_id: GameId) -> PostgresResult<u32> {
        get_game_service::get_game_num_players(&self.connection, game_id).await
    }

    /// Get a finished game.
    pub async fn get_finished_game(&self, game_id: GameId) -> PostgresResult<CompletedGame> {
        get_game_service::get_finished_game(&self.connection, game_id).await
    }

    /// Retrieve a game by its ID or None if it doesn't exist.
    pub async fn get_game(&self, game_id: GameId) -> PostgresResult<Option<Game>> {
        let game = get_game_service::get_game(&self.connection, game_id).await?;

        Ok(game.map(|g| Game {
            game_id: g.id,
            num_players: g.num_players as u32,
            started_at: g.started_at,
        }))
    }

    /// Retrieve all available games.
    pub async fn get_all_games(&self) -> PostgresResult<Vec<Game>> {
        let games = get_game_service::get_games(&self.connection).await?;
        Ok(games
            .into_iter()
            .map(|g| Game {
                game_id: g.id,
                num_players: g.num_players as u32,
                started_at: g.started_at,
            })
            .collect())
    }
}

/// A game.
#[derive(Debug, Clone)]
pub struct Game {
    /// The id of the game
    pub game_id: GameId,
    /// The number of players required for this game.
    pub num_players: u32,
    /// The time at which the game started or none if it has not yet started.
    pub started_at: Option<DateTime<Utc>>,
}
