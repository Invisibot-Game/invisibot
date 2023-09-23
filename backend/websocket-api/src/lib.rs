#![forbid(unsafe_code)]

#[doc(hidden)]
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use invisibot_client_api::game_message::GameMessage;
use invisibot_common::player_id::PlayerId;
use invisibot_game::{async_trait::async_trait, client_handler::ClientHandler};
#[doc(hidden)]
use serde::de::DeserializeOwned;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};
use tokio::{net::TcpStream, time::Instant};
#[doc(hidden)]
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

#[derive(Debug)]
pub struct WsHandler {
    clients: HashMap<PlayerId, WsClient>,
    spectators: Vec<WsClient>,
}

const DEFAULT_NEW_CLIENT_TIMEOUT_MILLIS: u64 = 400;

#[async_trait]
impl ClientHandler for WsHandler {
    async fn broadcast_message(&mut self, message: GameMessage) {
        for (_, client) in self.clients.iter_mut() {
            client.send_message(message.clone()).await;
        }
    }

    async fn broadcast_text(&mut self, message: String) {
        for (_, client) in self.clients.iter_mut() {
            client.send_text(message.clone()).await;
        }
    }

    async fn send_message(&mut self, player_id: &PlayerId, message: GameMessage) {
        self.clients
            .get_mut(player_id)
            .unwrap()
            .send_message(message)
            .await;
    }

    async fn receive_messages<ResponseMessage: DeserializeOwned + Send>(
        &mut self,
    ) -> HashMap<PlayerId, Option<ResponseMessage>> {
        futures::future::join_all(self.clients.iter_mut().map(|(id, client)| async {
            let response = client
                .receive_message(DEFAULT_NEW_CLIENT_TIMEOUT_MILLIS)
                .await;
            (*id, response)
        }))
        .await
        .into_iter()
        .collect()
    }

    async fn disconnect_player(&mut self, player_id: &PlayerId) {
        let mut p = self
            .clients
            .remove(player_id)
            .expect("Tried to disconnect nonexistant player");

        p.close().await;
    }

    async fn close(&mut self) {
        for (_, client) in self.clients.iter_mut() {
            client.close().await;
        }

        for client in self.spectators.iter_mut() {
            client.close().await;
        }
    }

    async fn broadcast_spectators(&mut self, message: GameMessage) {
        for client in self.spectators.iter_mut() {
            client.send_message(message.clone()).await;
        }
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
    incoming: SplitStream<WebSocketStream<TcpStream>>,
    outgoing: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl WsClient {
    pub async fn accept(stream: TcpStream) -> Self {
        let ws = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Failed to initiate websocket");

        let (outgoing, incoming) = ws.split();

        Self { incoming, outgoing }
    }

    pub async fn send_message(&mut self, message: GameMessage) {
        let serialized = serde_json::to_string(&message).expect("Failed to serialize message");

        if let Err(e) = self.outgoing.send(Message::text(serialized)).await {
            eprintln!("Failed to send message to clients, err: {e}");
        }
    }

    pub async fn send_text(&mut self, text: String) {
        if let Err(e) = self.outgoing.send(Message::Text(text)).await {
            eprintln!("Failed to send text message to clients, err: {e}");
        }
    }

    /// Cancelation safe
    pub async fn receive_message<ResponseMessage: DeserializeOwned>(
        &mut self,
        timeout_millis: u64,
    ) -> Option<ResponseMessage> {
        let response =
            tokio::time::timeout(Duration::from_millis(timeout_millis), self.incoming.next()).await;
        self.handle_message(response).await
    }

    pub async fn receive_message_until<ResponseMessage: DeserializeOwned>(
        &mut self,
        timeout_at: Instant,
    ) -> Option<ResponseMessage> {
        let response = tokio::time::timeout_at(timeout_at, self.incoming.next()).await;
        self.handle_message(response).await
    }

    /// Cancelation safe
    async fn handle_message<ResponseMessage: DeserializeOwned>(
        &mut self,
        response: Result<
            Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
            tokio::time::error::Elapsed,
        >,
    ) -> Option<ResponseMessage> {
        let message = match response {
            Ok(Some(Ok(mess))) => mess,
            Ok(Some(Err(e))) => {
                eprintln!("Failed to read message, error: {e}");
                return None;
            }
            Ok(None) => {
                eprintln!("Got empty from 'next'?");
                return None;
            }
            Err(_) => {
                eprintln!("No response within timelimit");
                return None;
            }
        };

        self.parse_message(message).await
    }

    /// Cancelation safe
    async fn parse_message<ResponseMessage: DeserializeOwned>(
        &mut self,
        message: Message,
    ) -> Option<ResponseMessage> {
        let text_response = message
            .to_text()
            .expect("Failed to read text from response");
        match serde_json::from_str(text_response) {
            Ok(m) => Some(m),
            Err(e) => {
                eprintln!("Failed to parse message [{text_response}], got err {e}");
                self.close().await;
                None
            }
        }
    }

    pub async fn close(&mut self) {
        println!("Closing WS connection");

        self.outgoing
            .close()
            .await
            .expect("Failed to close websocket stream");
    }
}
