use serde::{Deserialize, Serialize};

use crate::{game_logic::game_state::GameState, player::PlayerId};

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
        Self::GameRound(GameRound {
            game_state,
            player_id,
        })
    }

    pub fn goodbye(message: String) -> GameMessage {
        GameMessage::ClientGoodbye(message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRound {
    game_state: GameState,
    player_id: PlayerId,
}
