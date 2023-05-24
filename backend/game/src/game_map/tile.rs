use invisibot_common::{coordinate::Coordinate, tile_type::TileType};
use serde::{Deserialize, Serialize};

/// A tile on the game map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    /// The position of the tile.
    pub coord: Coordinate,
    /// What type of tile this is.
    pub tile_type: TileType,
}
