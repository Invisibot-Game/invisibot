#![forbid(unsafe_code)]

#[doc(hidden)]
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use invisibot_client_api::game_message::GameMessage;
use invisibot_common::player_id::PlayerId;
use invisibot_game::client_handler::ClientHandler;
#[doc(hidden)]
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use tokio::{net::TcpStream, runtime::Runtime};
#[doc(hidden)]
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

#[derive(Debug)]
pub struct WsHandler {
    clients: HashMap<PlayerId, WsClient>,
    spectators: Vec<WsClient>,
    rt: Runtime,
}

impl ClientHandler for WsHandler {
    fn broadcast_message(&mut self, message: GameMessage) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| self.rt.block_on(client.send_message(message.clone())))
    }

    fn broadcast_text(&mut self, message: String) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| self.rt.block_on(client.send_text(message.clone())))
    }

    fn send_message(&mut self, player_id: &PlayerId, message: GameMessage) {
        self.rt.block_on(
            self.clients
                .get_mut(player_id)
                .unwrap()
                .send_message(message),
        );
    }

    fn receive_messages<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> HashMap<PlayerId, Option<ResponseMessage>> {
        let messages: HashMap<PlayerId, Option<ResponseMessage>> = self
            .clients
            .iter_mut()
            .map(|(id, client)| {
                let response = self.rt.block_on(client.receive_message());
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

        self.rt.block_on(p.close());
    }

    fn close(&mut self) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| self.rt.block_on(client.close()));

        self.spectators
            .iter_mut()
            .for_each(|client| self.rt.block_on(client.close()));
    }

    fn broadcast_spectators(&mut self, message: GameMessage) {
        self.spectators
            .iter_mut()
            .for_each(|client| self.rt.block_on(client.send_message(message.clone())));
    }
}

impl WsHandler {
    pub fn add_player(&mut self, ws: WsClient) {
        let player_id = self.clients.len() as PlayerId;
        self.clients.insert(player_id, ws);
    }

    pub fn new_with_players(players: Vec<WsClient>, spectators: Vec<WsClient>) -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to get a reference to the current runtime");

        Self {
            clients: players
                .into_iter()
                .enumerate()
                .map(|(id, client)| (id as PlayerId, client))
                .collect(),
            spectators,
            rt,
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

    pub async fn receive_message<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> Option<ResponseMessage> {
        let message = match self.incoming.next().await {
            None => {
                eprintln!("Got empty from 'next'?");
                return None;
            }
            Some(Ok(mess)) => mess,
            Some(Err(e)) => {
                eprintln!("Failed to read message, error: {e}");
                return None;
            }
        };

        self.parse_message(message).await
    }

    /// Cancelation safe
    pub async fn try_receive_message<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> Option<ResponseMessage> {
        let message = match self.incoming.next().await? {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to read message, error: {e}");
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
