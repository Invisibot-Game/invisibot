use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    game_config::GameConfig,
    game_map::{game_map::GameMap, player::Player},
    utils::{coordinate::Coordinate, game_error::GameResult},
};

#[doc = "inline"]
pub mod completed_game;

/// A unique identifier for a game.
pub type GameId = Uuid;

/// A handler that is capable of storing games and retrieving them for later replay.
#[async_trait]
pub trait PersistenceHandler {
    /// Set the map to be used for the game with the provided `game_id`.
    async fn set_game_map(&self, game_id: GameId, map: GameMap) -> GameResult<()>;

    /// Save a finished round to the game.
    async fn save_round(
        &self,
        game_id: GameId,
        round_number: u32,
        players: Vec<Player>,
        shot_tiles: Vec<Coordinate>,
    ) -> GameResult<()>;

    /// Store that the game is finished.
    async fn game_done(&self, game_id: GameId) -> GameResult<()>;

    /// Retrieve the configuration for the provided `game_id`.
    async fn get_game_config(&self, game_id: GameId) -> GameResult<GameConfig>;
}
