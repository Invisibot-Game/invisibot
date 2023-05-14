use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{game_logic::game_state::GameState, utils::coordinate::Coordinate};

use super::player_id::PlayerId;

/// Messages sent from the server to the clients
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum GameMessage {
    /// A welcome message sent to the clients as they connect.
    ClientHello(String),
    /// A message sent on each game round requesting moves from the clients.
    GameRound(GameRound),
    /// A message sent as the game is over before the server closes the connection.
    ClientGoodbye(String),
    /// A player was killed.
    PlayerKilled(PlayerId),
    /// A player won
    PlayerWon(PlayerId),
}

impl GameMessage {
    /// Creates an instance of the ClientHello message.
    pub fn hello(message: String) -> Self {
        Self::ClientHello(message)
    }

    /// Creates an instance of the GameRound message.
    pub fn game_round(game_state: GameState, player_id: PlayerId) -> Self {
        Self::GameRound(GameRound::new(&game_state, &player_id))
    }

    /// Creates an instance of the ClientGoodbye message.
    pub fn goodbye(message: String) -> Self {
        Self::ClientGoodbye(message)
    }

    /// Sent between rounds if a player was killed last round.
    pub fn player_killed(player: PlayerId) -> Self {
        Self::PlayerKilled(player)
    }

    /// Sent to the player who won to inform them of that fact.
    pub fn player_won(player: PlayerId) -> Self {
        Self::PlayerWon(player)
    }

    /// Returns the message type in a human readable format.
    pub fn message_type(&self) -> String {
        match self {
            Self::ClientHello(_) => "Client Hello".to_string(),
            Self::GameRound(_) => "Game Round".to_string(),
            Self::ClientGoodbye(_) => "Client Goodbye".to_string(),
            Self::PlayerKilled(id) => format!("Player {id} was killed"),
            Self::PlayerWon(id) => format!("Player {id} won the game!"),
        }
    }
}

/// Information provided to each client on every round from which they can base their decision on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRound {
    /// The map width.
    pub width: u32,
    /// The map height.
    pub height: u32,
    /// A list of all coordinate that contain walls.
    pub walls: Vec<Coordinate>,
    /// A map of all players that became visible after the last move and their (then) coordinates.
    pub visible_players: HashMap<PlayerId, Coordinate>,
    /// The player id of the player this message is sent to.
    pub own_player_id: PlayerId,
}

impl GameRound {
    fn new(game_state: &GameState, current_player: &PlayerId) -> Self {
        let walls = game_state.map.get_wall_coords();

        let visible_players = game_state
            .players
            .iter()
            .filter(|&(id, p)| id == current_player || p.is_visible())
            .map(|(id, p)| (id.clone(), p.get_pos().clone()))
            .collect();

        Self {
            width: game_state.map.width,
            height: game_state.map.height,
            walls,
            visible_players,
            own_player_id: current_player.clone(),
        }
    }
}
