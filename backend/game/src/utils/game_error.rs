/// A generic game error
#[derive(thiserror::Error, Debug, Clone)]
pub enum GameError {
    /// A tile was requested which was outside of the game board bounds.
    #[error("Tile coordinate ({0}, {1}) is out of bounds!")]
    TileOutOfBounds(u32, u32),
    /// The gamestate is invalid in someway, preventing the game from continuing.
    #[error("Unexpected game state, err: `{0}`")]
    InvalidGameState(String),
}

#[doc = "hidden"]
pub type GameResult<T> = Result<T, GameError>;
