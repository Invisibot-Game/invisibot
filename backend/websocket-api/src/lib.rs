#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

use invisibot_game::{clients::ClientHandler, player::PlayerId};
use tungstenite::{accept, Message, WebSocket};

type WsClient = WebSocket<TcpStream>;

#[derive(Debug)]
pub struct WsHandler {
    server: TcpListener,
    clients: HashMap<PlayerId, WsClient>,
}

impl ClientHandler for WsHandler {
    fn accept_clients(&mut self, num_clients: usize) -> Vec<PlayerId> {
        for stream in self.server.incoming() {
            let websocket = accept(stream.unwrap()).unwrap();
            self.clients.insert(self.clients.len() as u32, websocket);
            if self.clients.len() >= num_clients {
                break;
            }
        }

        self.greet_clients();

        self.clients.iter().map(|(id, _)| id.clone()).collect()
    }

    fn send_text(&mut self, message: String) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| client.write_message(Message::text(&message)).unwrap());
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

    fn greet_clients(&mut self) {
        self.clients.iter_mut().for_each(|(id, ws)| {
            ws.write_message(Message::text(format!("Hello client {}!", id + 1)))
                .expect("Failed to send greeting to client!");
        })
    }
}
