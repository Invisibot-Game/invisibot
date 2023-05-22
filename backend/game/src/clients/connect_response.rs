use crate::persistence::GameId;
use serde::{Deserialize, Serialize};

/// A response expected to be sent when receiving a ClientHello message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResponse {
    /// The id of the game you wish to join.
    pub game_id: GameId,
    /// What type of player this is.
    pub client_type: ClientType,
}

/// Which type of client connected.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClientType {
    /// A player which is a part of the game.
    Player,
    /// A spectator who receives game updates but are not a part of the game.
    Spectator,
}
