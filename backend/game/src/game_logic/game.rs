use std::collections::HashMap;

use crate::{
    clients::{game_message::GameMessage, round_response::RoundResponse, ClientHandler},
    player::PlayerId,
    utils::game_error::GameResult,
};

use super::{game_config::GameConfig, game_state::GameState, player::PlayerClients};

pub struct Game<Handler: ClientHandler> {
    config: GameConfig,
    initial_state: GameState,
    game_rounds: Vec<GameState>,
    client_handler: Handler,
}

impl<T: ClientHandler> Game<T> {
    pub fn new(client_handler: T, game_config: GameConfig) -> Self {
        let mut client_handler = client_handler;
        let clients = client_handler.accept_clients(game_config.num_players);

        client_handler.broadcast_message(GameMessage::hello(format!("Welcome to the game!")));

        Self {
            initial_state: GameState::new(clients),
            game_rounds: vec![],
            client_handler,
            config: game_config,
        }
    }

    pub fn run_game(&mut self) -> GameResult<()> {
        let mut player_clients = PlayerClients::new();

        let mut states = vec![self.initial_state.clone()];
        let mut state: GameState = self.initial_state.clone();
        for _ in 0..(self.config.num_rounds - 1) {
            state.players.iter().for_each(|(id, _)| {
                self.client_handler
                    .send_message(id, GameMessage::game_round(state.clone(), id.clone()));
            });

            let moves: HashMap<PlayerId, RoundResponse> = self.client_handler.receive_messages();
            moves
                .into_iter()
                .for_each(|(id, resp)| println!("Player {id} repsonded with {resp:?}"));

            let new_state = state.run_round(&mut player_clients)?;
            states.push(state);
            state = new_state;
        }

        self.game_rounds = states;

        Ok(())
    }

    pub fn get_game_rounds(&self) -> Vec<GameState> {
        self.game_rounds.clone()
    }

    pub fn end_game(&mut self) {
        self.client_handler
            .broadcast_text(format!("Game is now closing, bye bye!"));
        self.client_handler.close();
    }
}
