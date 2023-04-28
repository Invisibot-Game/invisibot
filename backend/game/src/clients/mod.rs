use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::player::PlayerId;

use self::game_message::GameMessage;

pub mod game_message;
pub mod round_response;

pub trait ClientHandler {
    fn accept_clients(&mut self, num_clients: usize) -> Vec<PlayerId>;

    fn broadcast_message(&mut self, message: GameMessage);

    fn broadcast_text(&mut self, message: String);

    fn send_message(&mut self, player_id: &PlayerId, message: GameMessage);

    fn receive_messages<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> HashMap<PlayerId, ResponseMessage>;

    fn close(&mut self);
}
