use std::sync::Mutex;

use invisibot_game::game::Game;
use websocket_api::WsHandler;

#[derive(Default)]
pub struct CurrentGameState {
    pub game_mutex: Mutex<CurrentGame>,
}

#[derive(Default)]
pub struct CurrentGame {
    pub current_game: Option<RunningGame>,
}

pub struct RunningGame {
    pub game: Game<WsHandler>,
}
