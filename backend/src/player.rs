use std::collections::HashMap;

use crate::{
    coord,
    utils::{coordinate::Coordinate, direction::Direction},
};

pub type PlayerId = u32;

#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerId,
    pos: Coordinate,
}

impl Player {
    pub fn get_id(&self) -> &PlayerId {
        &self.id
    }

    pub fn get_pos(&self) -> &Coordinate {
        &self.pos
    }

    pub fn move_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.pos = coord!(self.pos.x, self.pos.y - 1),
            Direction::Down => self.pos = coord!(self.pos.x, self.pos.y + 1),
            Direction::Right => self.pos = coord!(self.pos.x + 1, self.pos.y),
            Direction::Left => self.pos = coord!(self.pos.x - 1, self.pos.y),
        }
    }
}

pub fn create_players() -> HashMap<PlayerId, Player> {
    let mut map = HashMap::new();

    let first_player_id: PlayerId = 0;
    map.insert(
        first_player_id,
        Player {
            id: first_player_id,
            pos: coord!(4, 4),
        },
    );

    let second_player_id: PlayerId = 1;
    map.insert(
        second_player_id,
        Player {
            id: second_player_id,
            pos: coord!(1, 2),
        },
    );

    map
}
