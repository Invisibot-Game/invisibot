use std::{collections::HashSet, path::Path};

use invisibot_common::{
    coord,
    coordinate::Coordinate,
    direction::Direction,
    game_error::{GameError, GameResult},
};
use serde::{Deserialize, Serialize};

use crate::game_map::{tile::Tile, tile_type::TileType};

/// The game map itself, where the game plays out.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMap {
    /// The width of the map.
    pub width: u32,
    /// The height of the map
    pub height: u32,
    starting_positions: Vec<Coordinate>,
    tiles: Vec<Tile>,
}

impl GameMap {
    /// Load the map from a bitmap image at the provided `map_path`.
    pub fn load_from_image(map_path: &Path) -> GameResult<Self> {
        let image = bmp::open(map_path).map_err(|e| {
            println!("Failed to read map file {e:?}");
            GameError::MapLoadError
        })?;

        let width = image.get_width();
        let height = image.get_height();

        let mut starting_positions: Vec<Coordinate> = vec![];

        let tiles = image
            .coordinates()
            .map(|(x, y)| {
                let pixel = image.get_pixel(x, y);
                let color = (pixel.r, pixel.g, pixel.b);
                if color == (0, 255, 0) {
                    starting_positions.push(coord!(x, y));
                }

                Tile {
                    coord: coord!(x, y),
                    tile_type: Self::tiletype_for_color(color),
                }
            })
            .collect();

        Ok(Self {
            tiles,
            starting_positions,
            width,
            height,
        })
    }

    fn tiletype_for_color(color: (u8, u8, u8)) -> TileType {
        match color {
            (0, 0, 0) => TileType::Wall,
            _ => TileType::Empty,
        }
    }

    /// Create a new GameMap from existing data.
    pub fn new(
        width: u32,
        height: u32,
        starting_positions: Vec<Coordinate>,
        wall_positions: HashSet<Coordinate>,
    ) -> Self {
        let tiles = (0..height)
            .into_iter()
            .map(|y| {
                (0..width)
                    .into_iter()
                    .map(move |x| coord!(x.clone(), y.clone()))
                    .map(|coord| {
                        let tile_type = if wall_positions.contains(&coord) {
                            TileType::Wall
                        } else {
                            TileType::Empty
                        };
                        Tile { coord, tile_type }
                    })
            })
            .flatten()
            .collect();

        Self {
            width,
            height,
            starting_positions,
            tiles,
        }
    }

    /// Returns the tile at the coordinate `coord` or an error if there is no such tile.
    pub fn get_tile_by_coord(&self, coord: &Coordinate) -> GameResult<Tile> {
        return self.get_tile(coord.x, coord.y);
    }

    /// Returns the tile at the coordinate of `(x, y)` or an error if there is no such tile.
    pub fn get_tile(&self, x: u32, y: u32) -> GameResult<Tile> {
        if x >= self.width || y >= self.height {
            return Err(GameError::TileOutOfBounds(x, y));
        }

        let index = (y * self.width + x) as usize;
        let tile = self
            .tiles
            .get(index)
            .ok_or(GameError::InvalidGameState(format!(
                "Expected tile at ({x},{y}) was not found!"
            )))?
            .clone();

        Ok(tile)
    }

    /// Returns the tile at the coordinate `coord` translated 1 step in the direction `dir`.
    pub fn get_tile_translated(&self, coord: &Coordinate, dir: &Direction) -> GameResult<Tile> {
        let translated_cord = match dir {
            Direction::Up => coord!(coord.x, coord.y - 1),
            Direction::Down => coord!(coord.x, coord.y + 1),
            Direction::Right => coord!(coord.x + 1, coord.y),
            Direction::Left => coord!(coord.x - 1, coord.y),
        };

        self.get_tile_by_coord(&translated_cord)
    }

    /// Returns the available starting positions of this map.
    pub fn get_starting_positions(&self) -> Vec<Coordinate> {
        self.starting_positions.clone()
    }

    /// Returns whether or not the tile at the position `pos` is walkable (contains a wall) or not.
    /// Returns false if there is no tile at the provided position.
    pub fn is_pos_walkable(&self, pos: &Coordinate) -> bool {
        let tile = if let Ok(tile) = self.get_tile_by_coord(pos) {
            tile
        } else {
            return false;
        };

        tile.tile_type != TileType::Wall
    }

    /// Returns a vec of the free tiles starting from the coordinate `coord` going in the direction `dir`.
    /// The returned vector does not contain the initial `coord`.
    pub fn get_line_of_sight(&self, coord: &Coordinate, dir: &Direction) -> Vec<Coordinate> {
        let mut curr_coord = coord.translate(dir);
        let mut coords = Vec::new();
        while self.is_pos_walkable(&curr_coord) {
            coords.push(curr_coord.clone());
            curr_coord = curr_coord.translate(dir);
        }

        coords
    }

    /// Returns a vec of all wall tiles in this map.
    pub fn get_wall_coords(&self) -> Vec<Coordinate> {
        self.tiles
            .iter()
            .filter(|t| t.tile_type == TileType::Wall)
            .map(|t| t.coord.clone())
            .collect()
    }
}
