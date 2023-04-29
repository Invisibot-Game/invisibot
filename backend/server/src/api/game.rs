use ::serde::{Deserialize, Serialize};
use invisibot_game::game_logic::{game::Game, game_config::GameConfig, game_map::TileType};
use rocket::{serde::json::Json, State};
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
        Err(e) => return GameResponse::err(format!("Failed to lock current_game, err: {e:?}")),
    };

    let running_game = match &mut curr_game.current_game {
        None => return GameResponse::err(format!("No game available")),
        Some(g) => g,
    };

    if let Err(e) = running_game.game.run_game() {
        return GameResponse::err(e.to_string());
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
) -> Result<String, String> {
    let mut curr_game = current_game
        .game_mutex
        .lock()
        .map_err(|e| format!("Failed to lock current_game state, err: {e:?}"))?;

    if curr_game.current_game.is_some() {
        return Err(format!("A game is already running"));
    }

    let ws = WsHandler::new(config.websocket_port);
    let game = Game::new(
        ws,
        GameConfig {
            num_players: request.num_players,
            num_rounds: request.num_rounds,
            map_dir: config.map_dir.clone(),
        },
    );

    curr_game.current_game = Some(RunningGame { game });

    Ok(format!("Game created!"))
}

#[delete("/game")]
pub fn delete_game(current_game: &State<CurrentGameState>) -> Result<String, String> {
    let mut curr_game = current_game
        .game_mutex
        .lock()
        .map_err(|e| format!("Failed to lock current game state, err: {e:?}"))?;

    match &mut curr_game.current_game {
        None => return Err(format!("No game is running!")),
        Some(_) => {}
    }

    curr_game.current_game = None;
    Ok(format!("Game deleted successfully"))
}
