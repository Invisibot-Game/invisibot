/// A generic game error
#[derive(thiserror::Error, Debug, Clone)]
pub enum GameError {
    /// A tile was requested which was outside of the game board bounds.
    #[error("Tile coordinate ({0}, {1}) is out of bounds!")]
    TileOutOfBounds(u32, u32),
    /// The gamestate is invalid in someway, preventing the game from continuing.
    #[error("Unexpected game state, err: `{0}`")]
    InvalidGameState(String),
    /// The map lacked enough starting locations for players.
    #[error("The map does not have enough starting locations to support the configured amount of players")]
    NotEnoughStartingPositions,
    /// Failed to load the map file
    #[error("Failed to load map file")]
    MapLoadError,
}

#[doc = "hidden"]
pub type GameResult<T> = Result<T, GameError>;
