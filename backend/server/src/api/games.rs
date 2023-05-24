use ::serde::{Deserialize, Serialize};
use invisibot_common::{coordinate::Coordinate, direction::Direction, GameId};
use invisibot_game::persistence::completed_game::{CompletedGame, RoundPlayer};
use invisibot_postgres::postgres_handler::PostgresHandler;
use rocket::{http::Status, serde::json::Json, State};
use uuid::Uuid;

use crate::{api::response::GameResponse, config::Config};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamesResponse {
    game_id: GameId,
    game_status: GameStatus,
    num_players: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameStatus {
    Running,
    WaitingForPlayers,
    Done,
}

#[get("/games")]
pub async fn get_games(pg_handler: &State<PostgresHandler>) -> GameResponse<Vec<GamesResponse>> {
    let games = match pg_handler.get_all_games().await {
        Ok(games) => games,
        Err(e) => {
            error!("Failed to get games, err: {e}");
            return GameResponse::internal_err();
        }
    };

    let games = games
        .into_iter()
        .map(|g| GamesResponse {
            game_id: g.game_id,
            game_status: if g.started_at.is_some() {
                if g.finished_at.is_some() {
                    GameStatus::Done
                } else {
                    GameStatus::Running
                }
            } else {
                GameStatus::WaitingForPlayers
            },
            num_players: g.num_players,
        })
        .collect();

    GameResponse::ok(games)
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsResponse {
    rounds: Vec<RoundResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundResponse {
    players: Vec<PlayerResponse>,
    width: u32,
    height: u32,
    wall_tiles: Vec<Coordinate>,
    shot_tiles: Vec<Coordinate>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    id: u32,
    x: u32,
    y: u32,
    rotation: Direction,
    visible: bool,
}

#[get("/games/<game_id>")]
pub async fn get_game(
    pg_handler: &State<PostgresHandler>,
    game_id: String,
) -> GameResponse<RoundsResponse> {
    let game_id = match Uuid::parse_str(&game_id) {
        Ok(id) => id,
        Err(e) => {
            println!("Failed to parse uuid, err: {e}");
            return GameResponse::err(Status::BadRequest, format!("Invalid game ID {game_id}"));
        }
    };

    let completed_game = match pg_handler.get_finished_game(game_id).await {
        Ok(g) => g,
        Err(e) => {
            println!("Failed to retrieve game, err: {e}");
            return GameResponse::err(
                Status::NotFound,
                format!("Game with ID {game_id} not found"),
            );
        }
    };

    let rounds = completed_game_to_rounds_response(completed_game);

    GameResponse::ok(RoundsResponse { rounds })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGameRequest {
    num_players: usize,
    num_rounds: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGameResponse {
    game_id: GameId,
}

#[post("/games", data = "<request>")]
pub async fn new_game(
    request: Json<NewGameRequest>,
    pg_handler: &State<PostgresHandler>,
    config: &State<Config>,
) -> GameResponse<NewGameResponse> {
    let game_id = match pg_handler
        .new_game(
            request.num_players as u32,
            request.num_rounds as u32,
            config.map_dir.clone(),
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to create new game, err: {e}");
            return GameResponse::internal_err();
        }
    };

    GameResponse::ok_with_status(NewGameResponse { game_id }, Status::Created)
}

fn completed_game_to_rounds_response(completed_game: CompletedGame) -> Vec<RoundResponse> {
    let rounds: Vec<RoundResponse> = completed_game
        .rounds
        .into_iter()
        .map(|round| RoundResponse {
            players: round
                .players
                .into_iter()
                .map(|p| to_player_response(p))
                .collect(),
            width: completed_game.map.width,
            height: completed_game.map.height,
            wall_tiles: completed_game.map.get_wall_coords(),
            shot_tiles: round.shot_tiles,
        })
        .collect();

    rounds
}

fn to_player_response(player: RoundPlayer) -> PlayerResponse {
    PlayerResponse {
        id: player.id,
        x: player.position.x,
        y: player.position.y,
        rotation: player.rotation,
        visible: player.visible,
    }
}
