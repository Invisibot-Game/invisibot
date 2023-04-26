#![forbid(unsafe_code)]

use api::game::{delete_game, get_game, new_game};
use config::Config;
use current_game::CurrentGameState;

#[macro_use]
extern crate rocket;

mod api;
mod config;
mod current_game;

#[launch]
async fn rocket() -> _ {
    let config = Config::new().expect("Failed to load config");

    rocket::build()
        .mount("/api", routes![get_game, new_game, delete_game])
        .manage(CurrentGameState::default())
        .manage(config)
}
