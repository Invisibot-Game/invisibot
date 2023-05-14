use ::serde::{Deserialize, Serialize};
use invisibot_game::{
    game::Game,
    game_config::GameConfig,
    persistence::{
        completed_game::{CompletedGame, RoundPlayer},
        GameId,
    },
    utils::{coordinate::Coordinate, direction::Direction},
};
use invisibot_postgres::{db_connection::DBConnection, postgres_handler::PostgresHandler};
use rocket::{http::Status, serde::json::Json, State};
use uuid::Uuid;
use websocket_api::WsHandler;

use crate::{api::response::GameResponse, config::Config};

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

#[get("/game/<game_id>")]
pub async fn get_game(
    db_connection: &State<DBConnection>,
    game_id: String,
) -> GameResponse<RoundsResponse> {
    let game_id = match Uuid::parse_str(&game_id) {
        Ok(id) => id,
        Err(e) => {
            println!("Failed to parse uuid, err: {e}");
            return GameResponse::err(Status::BadRequest, format!("Invalid game ID {game_id}"));
        }
    };

    let completed_game = match invisibot_postgres::get_game(db_connection, game_id).await {
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

#[post("/game", data = "<request>")]
pub async fn new_game(
    request: Json<NewGameRequest>,
    db_connection: &State<DBConnection>,
    config: &State<Config>,
) -> GameResponse<NewGameResponse> {
    let ws = WsHandler::new(config.websocket_port);
    let mut game = match Game::new(
        ws,
        PostgresHandler::new(&db_connection),
        GameConfig {
            num_players: request.num_players,
            num_rounds: request.num_rounds,
            map_dir: config.map_dir.clone(),
        },
    )
    .await
    {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to create a new game, err: {e:?}");
            return GameResponse::internal_err();
        }
    };

    match game.run_game().await {
        Ok(_) => {}
        Err(e) => {
            println!("An error occurred whilst simulating the game, err: {e}");
            game.cleanup();
            return GameResponse::err(Status::UnprocessableEntity, e.to_string());
        }
    }

    GameResponse::ok_with_status(
        NewGameResponse {
            game_id: game.get_id(),
        },
        Status::Created,
    )
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
