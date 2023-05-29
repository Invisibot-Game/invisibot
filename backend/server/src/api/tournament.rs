use std::collections::HashMap;

use ::serde::{Deserialize, Serialize};
use invisibot_common::{coordinate::Coordinate, direction::Direction, GameId, tournament::Tournament};
use invisibot_game::persistence::completed_game::{CompletedGame, RoundPlayer};
use invisibot_postgres::postgres_handler::{PostgresHandler};
use rocket::{http::Status, serde::json::Json, State};
use uuid::Uuid;

use crate::api::response::GameResponse;

#[derive(Debug, Clone, Serialize)]
pub struct GetTournamentsResponse {
    tournament_id: String,
}

#[get("/tournament")]
pub async fn get_tournaments(pg_handler: &State<PostgresHandler>) -> GameResponse<Vec<Tournament>> {
    match pg_handler.get_all_tournaments().await {
        Ok(ts) => GameResponse::ok(ts),
        Err(e) => GameResponse::internal_err(),
    }
}