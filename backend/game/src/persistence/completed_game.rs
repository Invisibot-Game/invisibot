use invisibot_common::{coordinate::Coordinate, direction::Direction, player_id::PlayerId, GameId};

use crate::game_map::game_map::GameMap;

/// Represents a completed game.
pub struct CompletedGame {
    /// The unique ID for the game.
    pub game_id: GameId,
    /// The map used for the game.
    pub map: GameMap,
    /// The rounds played during the game.
    pub rounds: Vec<GameRound>,
}

/// Represents a round in a finished game.
pub struct GameRound {
    /// A number unique for this round/game.
    pub round_number: u32,
    /// A list of all player states during this game.
    pub players: Vec<RoundPlayer>,
    /// The tiles that contained a shot during this turn.
    pub shot_tiles: Vec<Coordinate>,
}

/// Represents a player for a specific round.
pub struct RoundPlayer {
    /// A unique ID for this player for the game.
    pub id: PlayerId,
    /// The position the player had during the round.
    pub position: Coordinate,
    /// The rotation the player had during the round.
    pub rotation: Direction,
    /// Is true if the player was visible during the round.
    pub visible: bool,
}
