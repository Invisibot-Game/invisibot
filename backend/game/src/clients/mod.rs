use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};

use self::{game_message::GameMessage, player_id::PlayerId};

pub mod game_message;
pub mod player_id;
pub mod round_response;

pub trait ClientHandler {
    fn accept_clients(&mut self, num_clients: usize) -> HashSet<PlayerId>;

    fn broadcast_message(&mut self, message: GameMessage);

    fn broadcast_text(&mut self, message: String);

    fn send_message(&mut self, player_id: &PlayerId, message: GameMessage);

    fn receive_messages<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> HashMap<PlayerId, ResponseMessage>;

    fn close(&mut self);
}
