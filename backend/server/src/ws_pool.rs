use std::{collections::HashMap, ops::Add};
use tokio::{net::TcpListener, select};

use chrono::{DateTime, Duration, Utc};
use invisibot_client_api::{
    connect_response::{ClientType, ConnectResponse},
    game_message::GameMessage,
};
use invisibot_common::GameId;
use invisibot_game::game::Game;
use invisibot_postgres::postgres_handler::PostgresHandler;
use invisibot_websocket_api::{WsClient, WsHandler};
use tokio::task::{self, yield_now};

const CLIENT_TIMEOUT_MILLIS_DEFAULT: u32 = 2000;
const CLIENT_CONNECT_RESPONSE_TIMEOUT_MILLIS_CONFIG_KEY: &str =
    "client_connect_response_timeout_millis";

pub struct WsPoolManager {
    pg_handler: PostgresHandler,
    server: TcpListener,
    games: HashMap<GameId, GameSetup>,
    new_connections: Vec<NewConnection>,
}

pub struct NewConnection {
    timeout_at: DateTime<Utc>,
    client: WsClient,
}

enum NewConnectionResult {
    TimedOut {
        index: usize,
    },
    Responded {
        index: usize,
        response: ConnectResponse,
    },
    Nothing,
}

impl WsPoolManager {
    pub async fn init(pg_handler: PostgresHandler, websocket_port: u32) -> Self {
        let host = format!("0.0.0.0:{websocket_port}");
        println!("Setting up websocket connection on {host}");

        let server = TcpListener::bind(host)
            .await
            .expect("Failed to setup TCP listener");

        Self {
            pg_handler,
            server,
            games: HashMap::new(),
            new_connections: vec![],
        }
    }

    pub async fn start(mut self) {
        println!("Websocket pool starting up");
        loop {
            let server = &self.server;
            let new_connections = &mut self.new_connections;
            select! {
                conn = server.accept() => {
                    let mut client = match conn {
                        Ok((stream, _)) => {
                            println!("Client connecting!");
                            WsClient::accept(stream).await
                        }
                        Err(e) => {
                            panic!("An unexpected error occurred whilst listening for clients, err {e}")
                        }
                    };

                    // let mut client = self.accept_client().await;
                    client.send_message(GameMessage::ClientHello).await;
                    let timeout_at = self.timeout_at().await;

                    self.new_connections
                        .push(NewConnection { timeout_at, client });
                }
                result = check_new_connections(new_connections) => {
                    match result {
                        NewConnectionResult::Nothing => {},
                        NewConnectionResult::Responded { index, response } => {
                            let conn = self.new_connections.remove(index);
                            self.handle_new_client_response(conn.client, response)
                                .await;
                        }
                        NewConnectionResult::TimedOut { index } => {
                            let mut conn = self.new_connections.remove(index);
                            conn.client
                                .send_text(String::from("Timeout waiting for connection response"))
                                .await;
                            conn.client.close().await;
                        }
                    }
                }
            }

            yield_now().await;
        }
    }

    async fn timeout_at(&mut self) -> DateTime<Utc> {
        let millis = match self
            .pg_handler
            .get_config_u32(CLIENT_CONNECT_RESPONSE_TIMEOUT_MILLIS_CONFIG_KEY)
            .await
        {
            Ok(millis) => millis,
            Err(e) => {
                eprintln!("Failed to retrieve client timeout value, defaulting to {CLIENT_TIMEOUT_MILLIS_DEFAULT} millis (error: {e:?})");
                CLIENT_TIMEOUT_MILLIS_DEFAULT
            }
        };

        Utc::now().add(Duration::milliseconds(millis as i64))
    }

    async fn handle_new_client_response(
        &mut self,
        mut client: WsClient,
        connect_response: ConnectResponse,
    ) {
        let game_id = connect_response.game_id;

        let (num_players, curr_players) = if let Some(setup) = self.games.get_mut(&game_id) {
            // Add them to an existing game
            match connect_response.client_type {
                ClientType::Player => {
                    setup.curr_players.push(client);
                }
                ClientType::Spectator => {
                    setup.spectators.push(client);
                }
            }
            (setup.max_players as usize, setup.curr_players.len())
        } else {
            let game = match self.pg_handler.get_game(game_id).await {
                Ok(g) => g,
                Err(e) => {
                    println!("Failed to retrieve game from database, err: {e}");
                    client
                        .send_message(GameMessage::ServerError(
                            "Failed to retrieve game".to_string(),
                        ))
                        .await;
                    return;
                }
            };

            if let Some(game) = game {
                if game.started_at.is_some() {
                    client.send_message(GameMessage::GameStarted).await;
                    client.close().await;
                    return;
                }

                let mut players = vec![];
                let mut spectators = vec![];

                match connect_response.client_type {
                    ClientType::Player => {
                        players.push(client);
                    }
                    ClientType::Spectator => {
                        spectators.push(client);
                    }
                }

                self.games.insert(
                    connect_response.game_id,
                    GameSetup {
                        max_players: game.num_players,
                        curr_players: players,
                        spectators,
                    },
                );

                (game.num_players as usize, 1)
            } else {
                // Inform the client that there is no such game.
                client
                    .send_message(GameMessage::GameNotFound(game_id))
                    .await;
                client.close().await;
                return;
            }
        };

        if curr_players == num_players {
            println!("All players are in, starting game {game_id}");

            let mut setup = self.games.remove(&game_id).unwrap(); // Should always exist here
            if let Err(e) = self.pg_handler.set_game_started(game_id).await {
                println!("Failed to set game as started, err: {e}");
                setup.abort_game("Failed to start game").await;
                return;
            }

            let game_pg_handler = self.pg_handler.clone();
            task::spawn(play_game(game_id, setup, game_pg_handler));
        }
    }
}

async fn play_game(game_id: GameId, setup: GameSetup, pg_handler: PostgresHandler) {
    println!(
        "Starting game {game_id} with {} players",
        setup.curr_players.len()
    );
    let ws_handler = WsHandler::new_with_players(setup.curr_players, setup.spectators);
    let player_ids = ws_handler.get_player_ids();
    let mut game = Game::new(ws_handler, pg_handler, game_id, player_ids)
        .await
        .expect("Failed to create new game");
    game.run_game().await.expect("Failed to run game");
}

struct GameSetup {
    max_players: u32,
    curr_players: Vec<WsClient>,
    spectators: Vec<WsClient>,
}

impl GameSetup {
    async fn abort_game(&mut self, message: &str) {
        for client in self.curr_players.iter_mut() {
            client
                .send_message(GameMessage::ServerError(message.to_string()))
                .await;
            client.close().await;
        }
    }
}

async fn check_new_connections(new_connections: &mut Vec<NewConnection>) -> NewConnectionResult {
    let now = Utc::now();
    for (index, conn) in new_connections.iter_mut().enumerate() {
        match conn.client.try_receive_message().await {
            Some(m) => return NewConnectionResult::Responded { index, response: m },
            None => {
                if conn.timeout_at < now {
                    return NewConnectionResult::TimedOut { index };
                }
                continue;
            }
        };
    }

    NewConnectionResult::Nothing
}
