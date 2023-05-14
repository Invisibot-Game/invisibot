use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    game_map::{game_map::GameMap, player::Player},
    utils::{coordinate::Coordinate, game_error::GameResult},
};

use self::completed_game::CompletedGame;

#[doc = "inline"]
pub mod completed_game;

/// A unique identifier for a game.
pub type GameId = Uuid;

/// A handler that is capable of storing games and retrieving them for later replay.
#[async_trait]
pub trait PersistenceHandler {
    /// Store a new game that hasn't begun yet returning a unique ID for future reference.
    async fn new_game(&self, map: GameMap) -> GameResult<GameId>;

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

    /// Get a finished game by its ID.
    async fn get_game(&self, game_id: GameId) -> GameResult<CompletedGame>;
}
