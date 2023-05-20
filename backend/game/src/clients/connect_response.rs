use serde::{Deserialize, Serialize};

use crate::persistence::GameId;

/// A response expected to be sent when receiving a ClientHello message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectResponse {
    /// The id of the game you wish to join.
    pub game_id: GameId,
}
