use sqlx::{Pool, Postgres, Transaction};

use crate::postgres_error::{PostgresError, PostgresResult};

pub mod game_repository;
pub mod map_repository;
pub mod map_wall_repository;
pub mod player_repository;
pub mod round_repository;
pub mod shot_tile_repository;
pub mod starting_position_repository;
pub mod tournament_repository;

pub type DB = Postgres;

pub async fn new_transaction(db_pool: &Pool<DB>) -> PostgresResult<Transaction<'_, DB>> {
    match db_pool.begin().await {
        Ok(transaction) => Ok(transaction),
        Err(err) => {
            println!("Failed to create transaction: {:?}", err);
            Err(PostgresError::SqlxError(err))
        }
    }
}
