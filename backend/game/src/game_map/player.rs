use invisibot_common::{coordinate::Coordinate, direction::Direction, player_id::PlayerId};
use serde::{Deserialize, Serialize};

/// A player in the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: PlayerId,
    pos: Coordinate,
    rotation: Direction,
    visible: bool,
}

impl Player {
    /// Create a new player.
    pub fn new(id: PlayerId, pos: Coordinate, visible: bool) -> Self {
        Self {
            id,
            pos,
            rotation: Direction::Up,
            visible,
        }
    }

    /// Returns the ID of this player.
    pub fn get_id(&self) -> &PlayerId {
        &self.id
    }

    /// Returns the current position of the player.
    pub fn get_pos(&self) -> &Coordinate {
        &self.pos
    }

    /// Returns the current rotation of the player.
    pub fn get_rotation(&self) -> &Direction {
        &self.rotation
    }

    /// Returns whether the player is currently visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Returns the player with an updated position and visibility.
    pub fn update_pos(&self, new_pos: Coordinate, visible: bool) -> Self {
        let rotation =
            Coordinate::dir_between(&self.pos, &new_pos).unwrap_or(self.rotation.clone());

        Self {
            id: self.id,
            pos: new_pos,
            rotation,
            visible,
        }
    }

    /// Returns the player with an updated rotation.
    pub fn rotate(&self, new_rotation: Direction) -> Self {
        Self {
            id: self.id,
            pos: self.pos.clone(),
            rotation: new_rotation,
            visible: true,
        }
    }

    /// Returns the player after having shot a shot (sets it to be visible).
    pub fn shoot(&self) -> Self {
        Self {
            id: self.id,
            pos: self.pos.clone(),
            rotation: self.rotation.clone(),
            visible: true,
        }
    }

    /// Returns the player with updated visibility.
    pub fn update_visibility(&self, visible: bool) -> Self {
        Self {
            id: self.id,
            pos: self.pos.clone(),
            rotation: self.rotation.clone(),
            visible,
        }
    }
}
