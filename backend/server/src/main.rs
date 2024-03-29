#![forbid(unsafe_code)]

use api::games::{game_options, get_game, get_games, new_game};
use api::maps::get_maps;
use config::Config;
use invisibot_postgres::{db_connection::DBConnection, postgres_handler::PostgresHandler};
use rocket::fs::FileServer;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
use tokio::task;
use ws_pool::WsPoolManager;

#[macro_use]
extern crate rocket;

mod api;
mod config;
mod ws_pool;

#[launch]
async fn rocket() -> _ {
    let config = Config::new().expect("Failed to load config");

    let database_connection =
        DBConnection::new(&config.database_url, config.log_db_statements).await;

    let db_clone = database_connection.clone();
    let port = config.websocket_port;

    task::spawn(start_ws_pool(db_clone, port));

    let postgres_handler = PostgresHandler::new(&database_connection);

    let mut rocket = rocket::build()
        .mount("/api/maps", FileServer::from(&config.map_dir))
        .mount(
            "/api",
            routes![get_game, new_game, get_games, get_maps, game_options],
        )
        .manage(postgres_handler);

    if config.development_mode {
        rocket = rocket.attach(Cors);
    }

    rocket.manage(config)
}

struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Set CORS header to 'allow *' in dev mode",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

async fn start_ws_pool(database_connection: DBConnection, port: u32) {
    let ws_pool = WsPoolManager::init(PostgresHandler::new(&database_connection), port);
    println!("Starting WS_POOL");
    ws_pool.start().await
}
