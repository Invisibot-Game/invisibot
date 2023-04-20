use crate::{game_map::GameMap, player::Player, utils::coordinate::Coordinate};

pub struct GameState {
    game_map: GameMap,
    players: Vec<Player>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            game_map: GameMap::new(32, 32),
            players: vec![
                Player {
                    pos: Coordinate { x: 5, y: 7 },
                },
                Player {
                    pos: Coordinate { x: 9, y: 24 },
                },
            ],
        }
    }
}

impl GameState {
    pub fn print_map(&self) {
        fn row_to_str(game_map: GameMap, y: u32) -> String {
            (0..game_map.width)
                .map(|x| {
                    game_map
                        .get_tile(x, y)
                        .expect("Failed to get tile")
                        .to_string()
                })
                .collect::<Vec<String>>()
                .join("")
        }

        let map = format!(
            "MAP:\n{}",
            (0..self.game_map.height)
                .map(|y| {})
                .collect::<Vec<String>>()
                .join("\n")
        );
        println!(map);
    }
}
