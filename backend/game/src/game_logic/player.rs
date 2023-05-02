use serde::{Deserialize, Serialize};

use crate::{
    clients::player_id::PlayerId,
    utils::{coordinate::Coordinate, direction::Direction},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: PlayerId,
    pos: Coordinate,
    rotation: Direction,
    visible: bool,
}

impl Player {
    pub fn new(id: PlayerId, pos: Coordinate, visible: bool) -> Self {
        Self {
            id,
            pos,
            rotation: Direction::Up,
            visible,
        }
    }

    pub fn get_id(&self) -> &PlayerId {
        &self.id
    }

    pub fn get_pos(&self) -> &Coordinate {
        &self.pos
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn update_pos(player: &Player, new_pos: Coordinate, visible: bool) -> Self {
        let rotation =
            Coordinate::dir_between(&player.pos, &new_pos).unwrap_or(player.rotation.clone());
        Self {
            id: player.id.clone(),
            pos: new_pos,
            rotation,
            visible,
        }
    }

    pub fn update_visibility(player: &Player, visible: bool) -> Self {
        Self {
            id: player.id.clone(),
            pos: player.pos.clone(),
            rotation: player.rotation.clone(),
            visible,
        }
    }
}
