use crate::player::PlayerId;

use self::game_message::GameMessage;

pub mod game_message;

pub trait ClientHandler {
    fn accept_clients(&mut self, num_clients: usize) -> Vec<PlayerId>;

    fn broadcast_message(&mut self, message: GameMessage);

    fn broadcast_text(&mut self, message: String);

    fn send_message(&mut self, player_id: &PlayerId, message: GameMessage);

    fn close(&mut self);
}
