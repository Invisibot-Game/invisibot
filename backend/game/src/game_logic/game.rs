use crate::clients::ClientHandler;

use super::{game_config::GameConfig, game_state::GameState};

pub struct Game<Handler: ClientHandler> {
    pub config: GameConfig,
    pub curr_state: GameState,
    pub prev_states: Vec<GameState>,
    pub client_handler: Handler,
}

impl<T: ClientHandler> Game<T> {
    pub fn new(client_handler: T, game_config: GameConfig) -> Self {
        let mut client_handler = client_handler;
        let clients = client_handler.accept_clients(game_config.num_players);

        Self {
            curr_state: GameState::new(clients),
            prev_states: vec![],
            client_handler,
            config: game_config,
        }
    }
}
