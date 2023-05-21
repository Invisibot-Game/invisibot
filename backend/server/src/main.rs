#![forbid(unsafe_code)]

use api::game::{get_game, new_game};
use config::Config;
use invisibot_postgres::{db_connection::DBConnection, postgres_handler::PostgresHandler};
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

    let database_connection = DBConnection::new(&config.database_url).await;

    let db_clone = database_connection.clone();
    let port = config.websocket_port.clone();

    task::spawn(start_ws_pool(db_clone, port));

    let mut rocket = rocket::build()
        .mount("/api", routes![get_game, new_game])
        .manage(database_connection);

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
    }
}

async fn start_ws_pool(database_connection: DBConnection, port: u32) {
    let ws_pool = WsPoolManager::init(PostgresHandler::new(&database_connection), port);
    println!("Starting WS_POOL");
    ws_pool.start().await
}
