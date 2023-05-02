use ::serde::{Deserialize, Serialize};
use invisibot_game::{game::Game, game_config::GameConfig, utils::tile_type::TileType};
use rocket::{http::Status, serde::json::Json, State};
use websocket_api::WsHandler;

use crate::{
    api::response::GameResponse,
    config::Config,
    current_game::{CurrentGameState, RunningGame},
};

#[derive(Debug, Clone, Serialize)]
pub struct RoundsResponse {
    rounds: Vec<RoundResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoundResponse {
    players: Vec<PlayerResponse>,
    width: u32,
    height: u32,
    tiles: Vec<TileType>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerResponse {
    id: u32,
    x: u32,
    y: u32,
    visible: bool,
}

#[get("/game")]
pub fn get_game(current_game: &State<CurrentGameState>) -> GameResponse<RoundsResponse> {
    let mut curr_game = match current_game.game_mutex.lock() {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to lock current_game, err: {e:?}");
            return GameResponse::internal_err();
        }
    };

    let running_game = match &mut curr_game.current_game {
        None => return GameResponse::err(Status::NotFound, format!("No game available")),
        Some(g) => g,
    };

    if let Err(e) = running_game.game.run_game() {
        error!("Failed to run game, err: {e:?}");
        return GameResponse::internal_err();
    }

    let rounds = running_game
        .game
        .get_game_rounds()
        .into_iter()
        .map(|s| RoundResponse {
            width: s.map.width,
            height: s.map.height,
            players: s
                .players
                .into_iter()
                .map(|(_, p)| {
                    let pos = p.get_pos();
                    PlayerResponse {
                        id: p.get_id().clone(),
                        x: pos.x,
                        y: pos.y,
                        visible: p.is_visible(),
                    }
                })
                .collect::<Vec<PlayerResponse>>(),
            tiles: s
                .map
                .tiles
                .into_iter()
                .map(|t| t.tile_type)
                .collect::<Vec<TileType>>(),
        })
        .collect::<Vec<RoundResponse>>();

    GameResponse::ok(RoundsResponse { rounds })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGameRequest {
    num_players: usize,
    num_rounds: usize,
}

#[post("/game", data = "<request>")]
pub fn new_game(
    request: Json<NewGameRequest>,
    current_game: &State<CurrentGameState>,
    config: &State<Config>,
) -> GameResponse<String> {
    let mut curr_game = match current_game.game_mutex.lock() {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to lock current_game state, err: {e:?}");
            return GameResponse::internal_err();
        }
    };

    if curr_game.current_game.is_some() {
        return GameResponse::err(Status::BadRequest, format!("A game is already running"));
    }

    let ws = WsHandler::new(config.websocket_port);
    let game = match Game::new(
        ws,
        GameConfig {
            num_players: request.num_players,
            num_rounds: request.num_rounds,
            map_dir: config.map_dir.clone(),
        },
    ) {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to create a new game, err: {e:?}");
            return GameResponse::internal_err();
        }
    };

    curr_game.current_game = Some(RunningGame { game });

    GameResponse::ok_with_status(format!("Game created!"), Status::Created)
}

#[delete("/game")]
pub fn delete_game(current_game: &State<CurrentGameState>) -> GameResponse<String> {
    let mut curr_game = match current_game.game_mutex.lock() {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to lock current game state, err: {e:?}");
            return GameResponse::internal_err();
        }
    };

    match &mut curr_game.current_game {
        None => return GameResponse::err(Status::BadRequest, String::from("No game is running")),
        Some(_) => {}
    }

    curr_game.current_game = None;
    GameResponse::ok(String::from("Game deleted successfully"))
}
