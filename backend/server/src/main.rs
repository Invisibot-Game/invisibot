#![forbid(unsafe_code)]

use api::game::{get_game, new_game};
use config::Config;
use invisibot_postgres::db_connection::DBConnection;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

#[macro_use]
extern crate rocket;

mod api;
mod config;

#[launch]
async fn rocket() -> _ {
    let config = Config::new().expect("Failed to load config");

    let database_connection = DBConnection::new(&config.database_url).await;

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
