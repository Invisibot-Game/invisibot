use crate::player::PlayerId;

pub mod game_message;

pub trait ClientHandler {
    fn accept_clients(&mut self, num_clients: usize) -> Vec<PlayerId>;

    fn send_text(&mut self, message: String);

    fn close(&mut self);
}
