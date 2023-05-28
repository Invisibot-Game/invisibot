use invisibot_common::game_error::GameError;

/// An error that can occurr in the postgres module.
#[derive(Debug, thiserror::Error)]
pub enum PostgresError {
    /// A database error occurred.
    #[error("Sqlx error, err `{0}`")]
    SqlxError(#[from] sqlx::Error),
    /// A game error occurred.
    #[error("Game error")]
    GameError(#[from] GameError),
    /// The game is not yet finished and cannot be retrieved until it is.
    #[error("Game not yet finished")]
    GameNotFinished,
}

/// A result with a postgres error.
pub type PostgresResult<T> = Result<T, PostgresError>;

impl From<PostgresError> for GameError {
    fn from(value: PostgresError) -> Self {
        match value {
            PostgresError::GameError(g) => g,
            err => {
                println!("A postgres error occurred {err}");
                Self::PersistanceError(err.to_string())
            }
        }
    }
}
