use std::collections::HashMap;

use crate::{
    coord,
    player::{create_players, Player, PlayerId},
    utils::{coordinate::Coordinate, direction::Direction},
};

use super::game_map::GameMap;

#[derive(Debug, Clone)]
pub struct GameState {
    pub map: GameMap,
    pub players: HashMap<PlayerId, Player>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            map: GameMap::new(7, 7),
            players: create_players(),
        }
    }

    pub fn get_player_by_coordinate(&self, coordinate: Coordinate) -> Option<Player> {
        if let Some(&p) = self
            .players
            .iter()
            .map(|(_, p)| p)
            .filter(|&p| p.get_pos().x == coordinate.x && p.get_pos().y == coordinate.y)
            .collect::<Vec<&Player>>()
            .first()
        {
            let p = p.clone();
            Some(p)
        } else {
            None
        }
    }

    pub fn print_map(&self) {
        let map_str = (0..self.map.height)
            .map(|y| {
                (0..self.map.width)
                    .map(|x| {
                        if let Some(p) = self.get_player_by_coordinate(coord!(x, y)) {
                            p.get_id().to_string()
                        } else {
                            self.map
                                .get_tile(x, y)
                                .expect("Failed to get tile")
                                .tile_type
                                .to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("MAP:\n{map_str}");
    }

    pub fn run_round(&self) -> Self {
        let next_round_players = self
            .players
            .iter()
            .map(|(id, p)| (id.clone(), p.clone()))
            .map(|(id, mut p)| {
                let dir = next_dir(&self.map, &p);
                p.move_dir(dir);
                (id, p)
            })
            .collect::<HashMap<PlayerId, Player>>();

        Self {
            map: self.map.clone(),
            players: next_round_players,
        }
    }
}

fn next_dir(map: &GameMap, player: &Player) -> Direction {
    let x = player.get_pos().x;
    let y = player.get_pos().y;

    let bottom_edge = map.height - 2;
    let right_edge = map.width - 2;

    match (x, y) {
        (1, 1) => Direction::Down,
        (1, _) if y == bottom_edge => Direction::Right,
        (1, _) => Direction::Down,
        (_, 1) => Direction::Left,
        (_, _) if x == right_edge => Direction::Up,
        (_, _) if y == bottom_edge => Direction::Right,
        (_, _) => Direction::Up,
    }
}
