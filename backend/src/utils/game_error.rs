#[derive(thiserror::Error, Debug, Clone)]
pub enum GameError {
    #[error("Tile coordinate ({0}, {1}) is out of bounds!")]
    TileOutOfBounds(u32, u32),
    #[error("Unexpected game state, err: `{0}`")]
    InvalidGameState(String),
}

pub type GameResult<T> = Result<T, GameError>;
