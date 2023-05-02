use std::collections::HashMap;

use crate::{
    clients::{
        game_message::GameMessage, player_id::PlayerId, round_response::RoundResponse,
        ClientHandler,
    },
    game_config::GameConfig,
    game_logic::game_state::GameState,
    utils::game_error::GameResult,
};

/// A particular game.
pub struct Game<Handler: ClientHandler> {
    config: GameConfig,
    initial_state: GameState,
    game_rounds: Vec<GameState>,
    client_handler: Handler,
}

impl<T: ClientHandler> Game<T> {
    /// Create a new game with the `client_handler` for communicating with clients and the settings specified in `game_config`.
    pub fn new(client_handler: T, game_config: GameConfig) -> GameResult<Self> {
        let mut client_handler = client_handler;
        let clients = client_handler.accept_clients(game_config.num_players);

        client_handler.broadcast_message(GameMessage::hello(format!("Welcome to the game!")));

        Ok(Self {
            initial_state: GameState::new(clients, &game_config.map_dir)?,
            game_rounds: vec![],
            client_handler,
            config: game_config,
        })
    }

    /// Simulate the entire game, mutating itself and storing all the states, use `get_game_rounds` to retrieve the states generated.
    pub fn run_game(&mut self) -> GameResult<()> {
        let mut states = vec![self.initial_state.clone()];
        let mut state: GameState = self.initial_state.clone();
        for _ in 0..(self.config.num_rounds - 1) {
            state.players.iter().for_each(|(id, _)| {
                self.client_handler
                    .send_message(id, GameMessage::game_round(state.clone(), id.clone()));
            });

            let moves: HashMap<PlayerId, RoundResponse> = self.client_handler.receive_messages();
            moves
                .iter()
                .for_each(|(id, resp)| println!("Player {id} responded with {resp:?}"));
            let moves = moves
                .into_iter()
                .map(|(id, resp)| (id, resp.get_dir()))
                .collect();

            let new_state = state.run_round(moves)?;
            states.push(state);
            state = new_state;
        }

        self.client_handler
            .broadcast_message(GameMessage::goodbye("Bye".to_string()));
        self.client_handler.close();
        self.game_rounds = states;

        Ok(())
    }

    /// Get the states that occurred during the game, will be empty if `run_game` is not called first.
    pub fn get_game_rounds(&self) -> Vec<GameState> {
        self.game_rounds.clone()
    }
}
