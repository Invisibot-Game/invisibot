use std::{
    collections::HashMap,
    io,
    net::{TcpListener, TcpStream},
};

use invisibot_game::{clients::game_message::GameMessage, persistence::GameId};
use invisibot_postgres::{
    db_connection::DBConnection,
    postgres_handler::{self, PostgresHandler},
};
use serde::{Deserialize, Serialize};
use tungstenite::WebSocket;
use websocket_api::WsClient;

pub struct WsPoolManager {
    pg_handler: PostgresHandler,
    server: TcpListener,
    games: HashMap<GameId, (u32, Vec<WsClient>)>,
}

impl WsPoolManager {
    pub fn init(pg_handler: PostgresHandler, websocket_port: u32) -> Self {
        let host = format!("0.0.0.0:{websocket_port}");
        println!("Setting up websocket connection on {host}");

        let server = TcpListener::bind(host).expect("Failed to setup TCP listener");
        server
            .set_nonblocking(true)
            .expect("Failed to set server to be non_blocking");

        Self {
            pg_handler,
            server,
            games: HashMap::new(),
        }
    }

    /// Must be run in a new thread!
    pub async fn start(self) {
        loop {
            let mut client = match self.server.accept() {
                Ok((stream, _)) => {
                    let mut ws = tungstenite::accept(stream).expect("Failed to initiate websocket");
                    WsClient::new(ws)
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // No connections available
                    continue;
                }
                Err(e) => {
                    panic!("An unexpected error occurred whilst listening for clients, err: {e}")
                }
            };

            let message = GameMessage::ClientHello;
            client.send_message(Message::CLientHello);
            let resp = client.receive_message::<ConnectResponse>();
            let game_id = resp.game_id;
            if let Some((num_players, _)) = self.games.get_mut(&game_id) {
                // Add them to an existing game
                game.push(client);
            } else {
                let num_players = self
                    .pg_handler
                    .get_num_players_for_game(game_id)
                    .await
                    .expect("Game not found");

                self.games
                    .insert(resp.game_id, (num_players.clone(), vec![]));
            };

            let (num_players, players) = self.games.get(&game_id).unwrap(); // Should always exist here
            if players.len() == num_players as usize {}
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ConnectResponse {
    game_id: GameId,
}

async fn play_game() {}
