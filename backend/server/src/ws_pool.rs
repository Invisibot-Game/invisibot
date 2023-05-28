use std::{collections::HashMap, net::TcpListener};

use invisibot_client_api::{
    connect_response::{ClientType, ConnectResponse},
    game_message::GameMessage,
};
use invisibot_common::GameId;
use invisibot_game::game::Game;
use invisibot_postgres::postgres_handler::PostgresHandler;
use invisibot_websocket_api::{WsClient, WsHandler};
use tokio::task::{self, yield_now};

pub struct WsPoolManager {
    pg_handler: PostgresHandler,
    server: TcpListener,
    games: HashMap<GameId, GameSetup>,
}

impl WsPoolManager {
    pub fn init(pg_handler: PostgresHandler, websocket_port: u32) -> Self {
        let host = format!("0.0.0.0:{websocket_port}");
        println!("Setting up websocket connection on {host}");

        let server = TcpListener::bind(host).expect("Failed to setup TCP listener");
        server
            .set_nonblocking(false)
            .expect("Failed to set server to be non_blocking");

        Self {
            pg_handler,
            server,
            games: HashMap::new(),
        }
    }

    pub async fn start(mut self) {
        println!("Websocket pool starting up");
        loop {
            self.handle_new_client().await;
            yield_now().await;
        }
    }

    async fn handle_new_client(&mut self) {
        let mut client = self.accept_client().await;

        client.send_message(GameMessage::ClientHello);

        // TODO: Maybe not wait here, maybe we store them in the state and check back later.
        let resp = client.receive_message::<ConnectResponse>();
        let game_id = resp.game_id;

        let (num_players, curr_players) = if let Some(setup) = self.games.get_mut(&game_id) {
            // Add them to an existing game
            match resp.client_type {
                ClientType::Player => {
                    setup.curr_players.push(client);
                }
                ClientType::Spectator => {
                    setup.spectators.push(client);
                }
            }
            (setup.max_players.clone() as usize, setup.curr_players.len())
        } else {
            let game = match self.pg_handler.get_game(game_id.clone()).await {
                Ok(g) => g,
                Err(e) => {
                    println!("Failed to retrieve game from database, err: {e}");
                    client.send_message(GameMessage::ServerError(
                        "Failed to retrieve game".to_string(),
                    ));
                    return;
                }
            };

            if let Some(game) = game {
                if game.started_at.is_some() {
                    client.send_message(GameMessage::GameStarted);
                    client.close();
                    return;
                }

                let mut players = vec![];
                let mut spectators = vec![];

                match resp.client_type {
                    ClientType::Player => {
                        players.push(client);
                    }
                    ClientType::Spectator => {
                        spectators.push(client);
                    }
                }

                self.games.insert(
                    resp.game_id,
                    GameSetup {
                        max_players: game.num_players.clone(),
                        curr_players: players,
                        spectators: spectators,
                    },
                );

                (game.num_players as usize, 1)
            } else {
                // Inform the client that there is no such game.
                client.send_message(GameMessage::GameNotFound(game_id));
                client.close();
                return;
            }
        };

        if curr_players == num_players {
            println!("All players are in, starting game {game_id}");
            let mut setup = self.games.remove(&game_id).unwrap(); // Should always exist here
            if let Err(e) = self.pg_handler.set_game_started(game_id.clone()).await {
                println!("Failed to set game as started, err: {e}");
                setup.abort_game("Failed to start game");
                return;
            }

            let game_pg_handler = self.pg_handler.clone();
            task::spawn(play_game(game_id, setup, game_pg_handler));
        }
    }

    async fn accept_client(&self) -> WsClient {
        println!("Accept client");
        match self.server.accept() {
            Ok((stream, _)) => {
                println!("Client connecting!");
                WsClient::accept(stream)
            }
            Err(e) => panic!("An unexpected error occurred whilst listening for clients, err {e}"),
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
    fn abort_game<'a>(&mut self, message: &'a str) {
        self.curr_players.iter_mut().for_each(|c| {
            c.send_message(GameMessage::ServerError(message.to_string()));
            c.close();
        })
    }
}
