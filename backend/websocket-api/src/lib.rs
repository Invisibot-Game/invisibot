#![forbid(unsafe_code)]

#[doc(hidden)]
use std::{
    collections::{HashMap, HashSet},
    net::TcpStream,
};

use invisibot_game::clients::{game_message::GameMessage, player_id::PlayerId, ClientHandler};
#[doc(hidden)]
use serde::de::DeserializeOwned;
#[doc(hidden)]
use tungstenite::{Message, WebSocket};

#[derive(Debug)]
pub struct WsHandler {
    clients: HashMap<PlayerId, WsClient>,
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
    ) -> HashMap<PlayerId, ResponseMessage> {
        self.clients
            .iter_mut()
            .map(|(id, client)| {
                let response = client.receive_message();
                (id.clone(), response)
            })
            .collect()
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
            .for_each(|(_, client)| client.close())
    }
}

impl WsHandler {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, ws: WsClient) {
        let player_id = self.clients.len() as PlayerId;
        self.clients.insert(player_id, ws);
    }

    pub fn new_with_players(players: Vec<WsClient>) -> Self {
        Self {
            clients: players
                .into_iter()
                .enumerate()
                .map(|(id, client)| (id as PlayerId, client))
                .collect(),
        }
    }

    pub fn get_player_ids(&self) -> HashSet<PlayerId> {
        self.clients.iter().map(|(id, _)| id.clone()).collect()
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
            .write_message(Message::text(&serialized))
            .expect("Failed to send message")
    }

    pub fn send_text(&mut self, text: String) {
        self.conn.write_message(Message::Text(text)).unwrap();
    }

    pub fn receive_message<ResponseMessage: DeserializeOwned>(&mut self) -> ResponseMessage {
        let response = self.conn.read_message().unwrap();
        let text_response = response
            .to_text()
            .expect("Failed to read text from response");
        serde_json::from_str(text_response).expect("Failed to parse json from response")
    }

    pub fn close(&mut self) {
        self.conn
            .close(None)
            .expect("Dammit, failed to disconnect player");
    }
}
