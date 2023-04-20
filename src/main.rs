use game_map::GameMap;

mod game_error;
mod game_map;
mod game_state;
mod player;
mod utils;

fn main() {
    let map = GameMap::new(20, 20);
    map.print();
}
