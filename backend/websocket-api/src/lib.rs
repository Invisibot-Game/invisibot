#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

use invisibot_game::player::PlayerId;
use tungstenite::{accept, Message, WebSocket};

type WsClient = WebSocket<TcpStream>;

#[derive(Debug)]
pub struct WsHandler {
    clients: HashMap<PlayerId, WsClient>,
}

impl WsHandler {
    pub fn start(num_clients: usize) -> WsHandler {
        println!("Setting up websocket connections");
        let server = TcpListener::bind("0.0.0.0:4900").expect("Failed to bind to port");

        let mut clients: HashMap<PlayerId, WsClient> = HashMap::new();

        for stream in server.incoming() {
            let websocket = accept(stream.unwrap()).unwrap();
            clients.insert(clients.len() as u32, websocket);
            if clients.len() >= num_clients {
                break;
            }
        }

        let mut handler = WsHandler { clients };
        greet_clients(&mut handler);
        handler
    }

    pub fn send_message(&mut self, message: String) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| client.write_message(Message::text(&message)).unwrap());
    }

    pub fn close(&mut self) {
        self.clients
            .iter_mut()
            .for_each(|(_, client)| client.close(None).unwrap())
    }
}

fn greet_clients(handler: &mut WsHandler) {
    handler.clients.iter_mut().for_each(|(id, ws)| {
        ws.write_message(Message::text(format!("Hello client {}!", id + 1)))
            .expect("Failed to send greeting to client!");
    })
}
