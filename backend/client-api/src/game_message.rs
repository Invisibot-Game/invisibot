use invisibot_common::{coordinate::Coordinate, player_id::PlayerId, GameId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {}

/// Messages sent from the server to the clients
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum GameMessage {
    /// A welcome message sent to the clients as they connect.
    ClientHello,
    /// Something went wrong on the server.
    ServerError(String),
    /// A message sent on each game round requesting moves from the clients.
    GameRound(GameRound),
    /// A message sent on each game round to all spectators.
    GameRoundSpectators(SpectatorRound),
    /// A message sent as the game is over before the server closes the connection.
    ClientGoodbye(String),
    /// A player was killed.
    PlayerKilled(PlayerId),
    /// A player won.
    PlayerWon(PlayerId),
    /// The game has already started.
    GameStarted,
    /// No game with the provided ID could be found.
    GameNotFound(GameId),
}

impl GameMessage {
    /// Returns the message type in a human readable format.
    pub fn message_type(&self) -> String {
        match self {
            Self::ClientHello => "Client Hello".to_string(),
            Self::ServerError(_) => "Server error".to_string(),
            Self::GameRound(_) => "Game Round".to_string(),
            Self::GameRoundSpectators(_) => "Game Round Spectator".to_string(),
            Self::ClientGoodbye(_) => "Client Goodbye".to_string(),
            Self::PlayerKilled(id) => format!("Player {id} was killed"),
            Self::PlayerWon(id) => format!("Player {id} won the game!"),
            Self::GameStarted => "The game has already started".to_string(),
            Self::GameNotFound(id) => format!("No game with id {id} could be found"),
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
    /// Create a new GameRound
    pub fn new(
        width: u32,
        height: u32,
        walls: Vec<Coordinate>,
        visible_players: HashMap<PlayerId, Coordinate>,
        own_player_id: PlayerId,
    ) -> Self {
        Self {
            width,
            height,
            walls,
            visible_players,
            own_player_id,
        }
    }

    // fn new(game_state: &GameState, current_player: &PlayerId) -> Self {
    //     let walls = game_state.map.get_wall_coords();

    //     let visible_players = game_state
    //         .players
    //         .iter()
    //         .filter(|&(id, p)| id == current_player || p.is_visible())
    //         .map(|(id, p)| (id.clone(), p.get_pos().clone()))
    //         .collect();

    //     Self {
    //         width: game_state.map.width,
    //         height: game_state.map.height,
    //         walls,
    //         visible_players,
    //         own_player_id: current_player.clone(),
    //     }
    // }
}

/// Data sent to all spectators every round.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectatorRound {
    /// The map width.
    pub width: u32,
    /// The map height.
    pub height: u32,
    /// A list of all coordinate that contain walls.
    pub walls: Vec<Coordinate>,
    /// A map of all players that became visible after the last move and their (then) coordinates.
    pub visible_players: HashMap<PlayerId, Coordinate>,
}

impl SpectatorRound {
    /// Create a new SpectatorRound
    pub fn new(
        width: u32,
        height: u32,
        walls: Vec<Coordinate>,
        visible_players: HashMap<PlayerId, Coordinate>,
    ) -> Self {
        Self {
            width,
            height,
            walls,
            visible_players,
        }
    }
}
