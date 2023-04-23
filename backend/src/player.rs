use std::collections::HashMap;

use crate::{coord, utils::coordinate::Coordinate};

pub type PlayerId = u32;

#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerId,
    pos: Coordinate,
    visible: bool,
}

impl Player {
    pub fn get_id(&self) -> &PlayerId {
        &self.id
    }

    pub fn get_pos(&self) -> &Coordinate {
        &self.pos
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn update_pos(player: &Player, new_pos: Coordinate) -> Self {
        Self {
            id: player.id.clone(),
            pos: new_pos,
            visible: player.visible,
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
            pos: coord!(4, 3),
            visible: false,
        },
    );

    let second_player_id: PlayerId = 1;
    map.insert(
        second_player_id,
        Player {
            id: second_player_id,
            pos: coord!(1, 2),
            visible: false,
        },
    );

    map
}
