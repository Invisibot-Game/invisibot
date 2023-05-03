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

    pub fn get_rotation(&self) -> &Direction {
        &self.rotation
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn update_pos(&self, new_pos: Coordinate, visible: bool) -> Self {
        let rotation =
            Coordinate::dir_between(&self.pos, &new_pos).unwrap_or(self.rotation.clone());
        Self {
            id: self.id.clone(),
            pos: new_pos,
            rotation,
            visible,
        }
    }

    pub fn rotate(&self, new_rotation: Direction) -> Self {
        Self {
            id: self.id.clone(),
            pos: self.pos.clone(),
            rotation: new_rotation,
            visible: true,
        }
    }

    pub fn shoot(&self) -> Self {
        Self {
            id: self.id.clone(),
            pos: self.pos.clone(),
            rotation: self.rotation.clone(),
            visible: true,
        }
    }

    pub fn update_visibility(&self, visible: bool) -> Self {
        Self {
            id: self.id.clone(),
            pos: self.pos.clone(),
            rotation: self.rotation.clone(),
            visible,
        }
    }
}
