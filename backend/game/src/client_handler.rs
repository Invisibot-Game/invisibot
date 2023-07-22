use invisibot_client_api::game_message::GameMessage;
use invisibot_common::player_id::PlayerId;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// A handler responsible for handling communication between the server and the clients.
pub trait ClientHandler {
    /// Broadcast the `message` to all clients.
    fn broadcast_message(&mut self, message: GameMessage);

    /// Broadcast the `message` string to all clients.
    fn broadcast_text(&mut self, message: String);

    /// Send the `message` to the player with id `player_id`.
    fn send_message(&mut self, player_id: &PlayerId, message: GameMessage);

    /// Receive messages of type `ResponseMessage` from all connected clients.
    fn receive_messages<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> HashMap<PlayerId, Option<ResponseMessage>>;

    /// Disconnect a player, typically happens when they die.
    fn disconnect_player(&mut self, player_id: &PlayerId);

    /// Broadcast a message but only to Spectator clients.
    fn broadcast_spectators(&mut self, message: GameMessage);

    /// Close connections (if applicable) to all clients.
    /// Indicates that there will be no more messages from the server.
    fn close(&mut self);
}
