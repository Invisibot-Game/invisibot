use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};

use self::{game_message::GameMessage, player_id::PlayerId};

#[doc = "inline"]
pub mod game_message;
#[doc = "inline"]
pub mod player_id;
#[doc = "inline"]
pub mod round_response;

/// A handler responsible for handling communication between the server and the clients.
pub trait ClientHandler {
    /// Accept `num_client` players for a game returning a list of their PlayerIds.
    /// Note that the returned vector must have length `num_clients` and each of the returned PlayerIds are required to be unique.
    fn accept_clients(&mut self, num_clients: usize) -> HashSet<PlayerId>;

    /// Broadcast the `message` to all clients.
    fn broadcast_message(&mut self, message: GameMessage);

    /// Broadcast the `message` string to all clients.
    fn broadcast_text(&mut self, message: String);

    /// Send the `message` to the player with id `player_id`
    fn send_message(&mut self, player_id: &PlayerId, message: GameMessage);

    /// Receive messages of type `ResponseMessage` from all connected clients.
    fn receive_messages<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> HashMap<PlayerId, ResponseMessage>;

    /// Close connections (if applicable) to all clients.
    /// Indicates that there will be no more messages from the server.
    fn close(&mut self);
}
