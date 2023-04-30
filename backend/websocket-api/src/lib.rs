#![forbid(unsafe_code)]

#[doc(hidden)]
use std::{
    collections::{HashMap, HashSet},
    net::{TcpListener, TcpStream},
};

use invisibot_game::clients::{game_message::GameMessage, player_id::PlayerId, ClientHandler};
#[doc(hidden)]
use serde::de::DeserializeOwned;
#[doc(hidden)]
use tungstenite::{accept, Message, WebSocket};

type WsClient = WebSocket<TcpStream>;

#[derive(Debug)]
pub struct WsHandler {
    server: TcpListener,
    clients: HashMap<PlayerId, WsClient>,
}

impl ClientHandler for WsHandler {
    fn accept_clients(&mut self, num_clients: usize) -> HashSet<PlayerId> {
        for stream in self.server.incoming() {
            let websocket = accept(stream.unwrap()).unwrap();
            self.clients.insert(self.clients.len() as u32, websocket);
            if self.clients.len() >= num_clients {
                break;
            }
        }

        self.clients.iter().map(|(id, _)| id.clone()).collect()
    }

    fn broadcast_message(&mut self, message: GameMessage) {
        let serialized = serde_json::to_string(&message).unwrap();

        self.clients.iter_mut().for_each(|(_, client)| {
            client
                .write_message(Message::text(&serialized))
                .expect("Failed to send message")
        })
    }

    fn broadcast_text(&mut self, message: String) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| client.write_message(Message::text(&message)).unwrap());
    }

    fn send_message(&mut self, player_id: &PlayerId, message: GameMessage) {
        let serialized = serde_json::to_string(&message).unwrap();
        self.clients
            .get_mut(player_id)
            .unwrap()
            .write_message(Message::text(&serialized))
            .expect("Failed to send message")
    }

    fn receive_messages<ResponseMessage: DeserializeOwned>(
        &mut self,
    ) -> HashMap<PlayerId, ResponseMessage> {
        self.clients
            .iter_mut()
            .map(|(id, client)| (id, client.read_message().unwrap()))
            .map(|(id, c)| {
                let text_response = c.to_text().expect("Failed to read text from response");
                let json = serde_json::from_str(text_response)
                    .expect("Failed to parse json from response");
                (id.clone(), json)
            })
            .collect()
    }

    fn close(&mut self) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| client.close(None).unwrap())
    }
}

impl WsHandler {
    pub fn new(websocket_port: u32) -> WsHandler {
        println!("Setting up websocket connections");
        let server =
            TcpListener::bind(format!("0.0.0.0:{websocket_port}")).expect("Failed to bind to port");

        WsHandler {
            server: server,
            clients: HashMap::new(),
        }
    }
}
