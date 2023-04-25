#![forbid(unsafe_code)]

use api::game::{delete_game, get_game, new_game};
use current_game::CurrentGameState;

#[macro_use]
extern crate rocket;

mod api;
mod current_game;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_game, new_game, delete_game])
        .manage(CurrentGameState::default())
}
