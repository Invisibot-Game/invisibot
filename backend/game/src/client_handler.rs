use async_trait::async_trait;
use invisibot_client_api::game_message::GameMessage;
use invisibot_common::player_id::PlayerId;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// A handler responsible for handling communication between the server and the clients.
#[async_trait]
pub trait ClientHandler {
    /// Broadcast the `message` to all clients.
    async fn broadcast_message(&mut self, message: GameMessage);

    /// Broadcast the `message` string to all clients.
    async fn broadcast_text(&mut self, message: String);

    /// Send the `message` to the player with id `player_id`.
    async fn send_message(&mut self, player_id: &PlayerId, message: GameMessage);

    /// Receive messages of type `ResponseMessage` from all connected clients.
    async fn receive_messages<ResponseMessage: DeserializeOwned + Send>(
        &mut self,
    ) -> HashMap<PlayerId, Option<ResponseMessage>>;

    /// Disconnect a player, typically happens when they die.
    async fn disconnect_player(&mut self, player_id: &PlayerId);

    /// Broadcast a message but only to Spectator clients.
    async fn broadcast_spectators(&mut self, message: GameMessage);

    /// Close connections (if applicable) to all clients.
    /// Indicates that there will be no more messages from the server.
    async fn close(&mut self);
}
