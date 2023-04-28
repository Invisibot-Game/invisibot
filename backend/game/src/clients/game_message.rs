use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    game_logic::{game_map::TileType, game_state::GameState},
    player::PlayerId,
    utils::coordinate::Coordinate,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum GameMessage {
    ClientHello(String),
    GameRound(GameRound),
    ClientGoodbye(String),
}

impl GameMessage {
    pub fn hello(message: String) -> GameMessage {
        GameMessage::ClientHello(message)
    }

    pub fn game_round(game_state: GameState, player_id: PlayerId) -> GameMessage {
        Self::GameRound(GameRound::new(&game_state, &player_id))
    }

    pub fn goodbye(message: String) -> GameMessage {
        GameMessage::ClientGoodbye(message)
    }

    /// Returns the message type in a human readable format.
    pub fn message_type(&self) -> String {
        String::from(match self {
            GameMessage::ClientHello(_) => "Client Hello",
            GameMessage::GameRound(_) => "Game Round",
            GameMessage::ClientGoodbye(_) => "Client Goodbye",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRound {
    pub width: u32,
    pub height: u32,
    pub walls: Vec<Coordinate>,
    pub visible_players: HashMap<PlayerId, Coordinate>,
    pub own_player_id: PlayerId,
}

impl GameRound {
    pub fn new(game_state: &GameState, current_player: &PlayerId) -> Self {
        Self {
            width: game_state.map.width,
            height: game_state.map.height,
            walls: game_state
                .map
                .tiles
                .iter()
                .filter(|t| t.tile_type == TileType::Wall)
                .map(|t| t.coord.clone())
                .collect(),
            visible_players: game_state
                .players
                .iter()
                .filter(|&(id, p)| id == current_player || p.is_visible())
                .map(|(id, p)| (id.clone(), p.get_pos().clone()))
                .collect(),
            own_player_id: current_player.clone(),
        }
    }
}
