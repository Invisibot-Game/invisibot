use std::sync::Mutex;

use invisibot_game::game_logic::game_state::GameState;
use websocket_api::WsHandler;

#[derive(Default, Debug)]
pub struct CurrentGameState {
    pub game_mutex: Mutex<CurrentGame>,
}

#[derive(Default, Debug)]
pub struct CurrentGame {
    pub game: Option<RunningGame>,
}

#[derive(Debug)]
pub struct RunningGame {
    pub handler: WsHandler,
    pub game_state: GameState,
}
