use serde::{Deserialize, Serialize};

use crate::utils::coordinate::Coordinate;

use super::tile_type::TileType;

/// A tile on the game map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    /// The position of the tile.
    pub coord: Coordinate,
    /// What type of tile this is.
    pub tile_type: TileType,
}
