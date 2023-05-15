use invisibot_game::persistence::GameId;
use invisibot_postgres::db_connection::DBConnection;
use rocket::tokio::net::TcpListener;

type WsClient = WebSocket<TcpStream>;

#[derive(Debug, Clone)]
pub struct WsPoolManager {
    db_connection: DBConnection,
    server: TcpListener,
    games: HashMap<GameId, Vec<WsClient>>,
    unknown_clients: Vec<WsClient>,
}

impl WsPoolManager {
    pub fn init(db_connection: DBConnection, websocket_port: u32) -> Self {
        let host = format!("0.0.0.0:{websocket_port}");
        println!("Setting up websocket connection on {host}");

        let server = TcpListener::bind(host).expect("Failed to setup TCP listener");
        server
            .set_nonblocking(true)
            .expect("Failed to set server to be non_blocking");

        Self {
            db_connection,
            server,
            games: HashMap::new(),
            unknown_clients: vec![],
        }
    }

    /// Must be run in a new thread!
    pub fn start(self) {
        loop {
            // self.server.
        }
    }
}
