#![forbid(unsafe_code)]

use api::get_game::get_game;
#[macro_use]
extern crate rocket;

mod api;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_game])
}
