use invisibot_game::{
    game_logic::{game_map::TileType, game_state::GameState, player::PlayerClients},
    utils::game_error::GameResult,
};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use websocket_api::WsHandler;

use crate::{
    api::response::GameResponse,
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

    let game = match &mut curr_game.game {
        None => return GameResponse::err(format!("No game available")),
        Some(g) => g,
    };

    game.handler.send_message(format!("Game will now run!"));

    let states = match run_game(game.game_state.clone()) {
        Ok(s) => s,
        Err(e) => return GameResponse::err(e.to_string()),
    };

    let rounds = states
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
pub struct NewGameRequest {
    num_players: usize,
}

#[post("/game", data = "<request>")]
pub fn new_game(
    request: Json<NewGameRequest>,
    current_game: &State<CurrentGameState>,
) -> Result<String, String> {
    let mut curr_game = current_game
        .game_mutex
        .lock()
        .map_err(|e| format!("Failed to lock current_game state, err: {e:?}"))?;

    if curr_game.game.is_some() {
        return Err(format!("A game is already running"));
    }

    let ws = WsHandler::start(request.num_players);
    let game_state = GameState::new();

    curr_game.game = Some(RunningGame {
        handler: ws,
        game_state,
    });

    Ok(format!("Game created!"))
}

#[delete("/game")]
pub fn delete_game(current_game: &State<CurrentGameState>) -> Result<String, String> {
    let mut curr_game = current_game
        .game_mutex
        .lock()
        .map_err(|e| format!("Failed to lock current game state, err: {e:?}"))?;

    match &mut curr_game.game {
        None => return Err(format!("No game is running!")),
        Some(g) => {
            g.handler
                .send_message(format!("Game is now closing, bye bye!"));
            g.handler.close();
        }
    }

    curr_game.game = None;
    Ok(format!("Game deleted successfully"))
}

fn run_game(initial_state: GameState) -> GameResult<Vec<GameState>> {
    let mut player_clients = PlayerClients::new();

    let mut states = vec![initial_state.clone()];
    let mut state: GameState = initial_state;
    for _ in 1..7 {
        let new_state = state.run_round(&mut player_clients)?;
        states.push(state);
        state = new_state;
    }

    Ok(states)
}
