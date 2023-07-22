#![forbid(unsafe_code)]

#[doc(hidden)]
use std::{
    collections::{HashMap, HashSet},
    net::TcpStream,
};

use invisibot_client_api::game_message::GameMessage;
use invisibot_common::player_id::PlayerId;
use invisibot_game::client_handler::ClientHandler;
#[doc(hidden)]
use serde::de::DeserializeOwned;
#[doc(hidden)]
use tungstenite::{Message, WebSocket};

#[derive(Debug)]
pub struct WsHandler {
    clients: HashMap<PlayerId, WsClient>,
    spectators: Vec<WsClient>,
}

impl ClientHandler for WsHandler {
    fn broadcast_message(&mut self, message: GameMessage) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| client.send_message(message.clone()))
    }

    fn broadcast_text(&mut self, message: String) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| client.send_text(message.clone()))
    }

    fn send_message(&mut self, player_id: &PlayerId, message: GameMessage) {
        self.clients
            .get_mut(player_id)
            .unwrap()
            .send_message(message);
    }

    fn receive_messages<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> HashMap<PlayerId, Option<ResponseMessage>> {
        let messages: HashMap<PlayerId, Option<ResponseMessage>> = self
            .clients
            .iter_mut()
            .map(|(id, client)| {
                let response = client.receive_message();
                (*id, response)
            })
            .collect();
        messages
            .iter()
            .filter(|(_, response)| response.is_none())
            .for_each(|(player_id, _)| self.disconnect_player(player_id));

        messages
    }

    fn disconnect_player(&mut self, player_id: &PlayerId) {
        let mut p = self
            .clients
            .remove(player_id)
            .expect("Tried to disconnect nonexistant player");

        p.close();
    }

    fn close(&mut self) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| client.close());

        self.spectators.iter_mut().for_each(|client| client.close());
    }

    fn broadcast_spectators(&mut self, message: GameMessage) {
        self.spectators
            .iter_mut()
            .for_each(|client| client.send_message(message.clone()));
    }
}

impl WsHandler {
    pub fn add_player(&mut self, ws: WsClient) {
        let player_id = self.clients.len() as PlayerId;
        self.clients.insert(player_id, ws);
    }

    pub fn new_with_players(players: Vec<WsClient>, spectators: Vec<WsClient>) -> Self {
        Self {
            clients: players
                .into_iter()
                .enumerate()
                .map(|(id, client)| (id as PlayerId, client))
                .collect(),
            spectators,
        }
    }

    pub fn get_player_ids(&self) -> HashSet<PlayerId> {
        self.clients.keys().copied().collect()
    }
}

#[derive(Debug)]
pub struct WsClient {
    conn: WebSocket<TcpStream>,
}

impl WsClient {
    pub fn accept(stream: TcpStream) -> Self {
        let ws = tungstenite::accept(stream).expect("Failed to initiate websocket");
        Self { conn: ws }
    }

    pub fn send_message(&mut self, message: GameMessage) {
        let serialized = serde_json::to_string(&message).unwrap();
        self.conn
            .write_message(Message::text(serialized))
            .expect("Failed to send message")
    }

    pub fn send_text(&mut self, text: String) {
        self.conn.write_message(Message::Text(text)).unwrap();
    }

    pub fn receive_message<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> Option<ResponseMessage> {
        let response = self.conn.read_message().unwrap();
        let text_response = response
            .to_text()
            .expect("Failed to read text from response");
        match serde_json::from_str(text_response) {
            Ok(m) => Some(m),
            Err(e) => {
                eprintln!("Failed to parse message [{text_response}], got err {e}");
                self.close();
                None
            }
        }
    }

    pub fn close(&mut self) {
        println!("Closing WS connection");
        self.conn
            .close(None)
            .expect("Dammit, failed to disconnect player");
    }
}
