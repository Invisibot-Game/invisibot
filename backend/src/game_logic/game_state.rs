use std::collections::HashMap;

use crate::{
    player::{create_players, Player, PlayerId},
    utils::{coordinate::Coordinate, direction::Direction, game_error::GameResult},
};

use super::game_map::{GameMap, TileType};

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

    pub fn get_player_by_coord(&self, coord: &Coordinate) -> Option<Player> {
        if let Some(&p) = self
            .players
            .iter()
            .map(|(_, p)| p)
            .filter(|&p| p.get_pos().x == coord.x && p.get_pos().y == coord.y)
            .collect::<Vec<&Player>>()
            .first()
        {
            let p = p.clone();
            Some(p)
        } else {
            None
        }
    }

    pub fn run_round(&self) -> GameResult<Self> {
        let next_round_players = self
            .players
            .iter()
            .map(|(id, p)| (id.clone(), p.clone()))
            .map(|(id, mut p)| {
                let dir = next_dir(&self.map, &p);

                let new_tile = self
                    .map
                    .get_tile_translated(p.get_pos(), &dir)
                    .unwrap_or(self.map.get_tile_by_coord(p.get_pos())?);

                if self.is_pos_free(&new_tile.coord) {
                    p.move_to(new_tile.coord);
                }
                Ok((id, p))
            })
            .collect::<GameResult<HashMap<PlayerId, Player>>>()?;

        Ok(Self {
            map: self.map.clone(),
            players: next_round_players,
        })
    }

    fn is_pos_free(&self, pos: &Coordinate) -> bool {
        if let Some(_) = self.get_player_by_coord(pos) {
            return false;
        }

        let tile = if let Ok(tile) = self.map.get_tile_by_coord(pos) {
            tile
        } else {
            return false;
        };

        tile.tile_type != TileType::Wall
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
