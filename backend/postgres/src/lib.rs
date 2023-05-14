#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Persistence handler for postgres, storing games in a postgresql database.

use db_connection::DBConnection;
use invisibot_game::{
    persistence::{completed_game::CompletedGame, GameId},
    utils::game_error::GameResult,
};
use services::get_game_service;

#[doc = "inline"]
pub mod db_connection;
#[doc = "inline"]
pub mod postgres_handler;

mod postgres_error;
mod repositories;
mod services;
mod tables;

/// Get a finished game from the database by ID.
pub async fn get_game(connection: &DBConnection, game_id: GameId) -> GameResult<CompletedGame> {
    get_game_service::get_finished_game(connection, game_id)
        .await
        .map_err(|e| e.into())
}
