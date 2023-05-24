use invisibot_common::game_error::GameError;

#[derive(Debug, thiserror::Error)]
pub enum PostgresError {
    #[error("Sqlx error, err `{0}`")]
    SqlxError(#[from] sqlx::Error),
    #[error("Game error")]
    GameError(#[from] GameError),
}

pub type PostgresResult<T> = Result<T, PostgresError>;

impl Into<GameError> for PostgresError {
    fn into(self) -> GameError {
        match self {
            Self::GameError(g) => g,
            err => {
                println!("A postgres error occurred {err}");
                GameError::PersistanceError(err.to_string())
            }
        }
    }
}
