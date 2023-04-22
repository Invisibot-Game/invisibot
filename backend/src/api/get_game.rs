use serde::Serialize;

use crate::{
    api::response::GameResponse,
    game_logic::{game_map::TileType, game_state::GameState},
    utils::game_error::GameResult,
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
}

#[get("/game")]
pub fn get_game() -> GameResponse<RoundsResponse> {
    let game_state = GameState::new();
    let states = match run_game(game_state) {
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

fn run_game(initial_state: GameState) -> GameResult<Vec<GameState>> {
    let mut states = vec![initial_state.clone()];
    let mut state: GameState = initial_state;
    for _ in 0..128 {
        let new_state = state.run_round()?;
        states.push(state);
        state = new_state;
    }

    Ok(states)
}
