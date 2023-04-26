use crate::utils::coordinate::Coordinate;

pub type PlayerId = u32;

#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerId,
    pos: Coordinate,
    visible: bool,
}

impl Player {
    pub fn new(id: PlayerId, pos: Coordinate, visible: bool) -> Self {
        Self { id, pos, visible }
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
        Self {
            id: player.id.clone(),
            pos: new_pos,
            visible,
        }
    }

    pub fn update_visibility(player: &Player, visible: bool) -> Self {
        Self {
            id: player.id.clone(),
            pos: player.pos.clone(),
            visible,
        }
    }
}
