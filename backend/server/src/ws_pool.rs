use std::{collections::HashMap, net::TcpListener};

use invisibot_game::{
    clients::{connect_response::ConnectResponse, game_message::GameMessage},
    game::Game,
    persistence::GameId,
};
use invisibot_postgres::postgres_handler::PostgresHandler;
use tokio::task::{self, yield_now};
use websocket_api::{WsClient, WsHandler};

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
            .set_nonblocking(false)
            .expect("Failed to set server to be non_blocking");

        Self {
            pg_handler,
            server,
            games: HashMap::new(),
        }
    }

    /// Must be run in a new thread!
    pub async fn start(mut self) {
        println!("Websocket pool starting up");
        loop {
            let mut client = self.accept_client().await;

            client.send_message(GameMessage::ClientHello);
            let resp = client.receive_message::<ConnectResponse>();
            let game_id = resp.game_id;

            let (num_players, curr_players) =
                if let Some((num_players, players)) = self.games.get_mut(&game_id) {
                    // Add them to an existing game
                    players.push(client);
                    (num_players.clone() as usize, players.len())
                } else {
                    // TODO: Check if the game has started and if so, don't allow connection.
                    let num_players = self
                        .pg_handler
                        .get_num_players_for_game(game_id)
                        .await
                        .expect("Game not found");

                    self.games
                        .insert(resp.game_id, (num_players.clone(), vec![client]));

                    (num_players as usize, 1)
                };

            if curr_players == num_players {
                // Start game
                // Mark the game as started somehow (in DB presumably)
                println!("All players are in, starting game {game_id}");
                let (_, players) = self.games.remove(&game_id).unwrap(); // Should always exist here
                self.pg_handler
                    .set_game_started(game_id.clone())
                    .await
                    .expect("Failed to set game as started");
                let game_pg_handler = self.pg_handler.clone();
                task::spawn(play_game(game_id, players, game_pg_handler));
            }

            yield_now().await;
        }
    }

    async fn accept_client(&self) -> WsClient {
        println!("Accept client");
        match self.server.accept() {
            Ok((stream, _)) => {
                println!("Client connecting!");
                let ws = tungstenite::accept(stream).expect("Failed to initiate websocket");
                WsClient::new(ws)
            }
            Err(e) => panic!("An unexpected error occurred whilst listening for clients, err {e}"),
        }
    }
}

async fn play_game(game_id: GameId, players: Vec<WsClient>, pg_handler: PostgresHandler) {
    println!("Starting game {game_id} with {} players", players.len());
    let ws_handler = WsHandler::new_with_players(players);
    let player_ids = ws_handler.get_player_ids();
    let mut game = Game::new(ws_handler, pg_handler, game_id, player_ids)
        .await
        .expect("Failed to create new game");
    game.run_game().await.expect("Failed to run game");
}
