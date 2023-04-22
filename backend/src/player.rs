use std::collections::HashMap;

use crate::{coord, utils::coordinate::Coordinate};

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

    pub fn move_to(&mut self, coord: Coordinate) {
        self.pos = coord;
    }
}

pub fn create_players() -> HashMap<PlayerId, Player> {
    let mut map = HashMap::new();

    let first_player_id: PlayerId = 0;
    map.insert(
        first_player_id,
        Player {
            id: first_player_id,
            pos: coord!(4, 3),
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
