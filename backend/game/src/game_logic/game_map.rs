use std::{fmt::Display, path::Path};

use bmp::Pixel;
use serde::{Deserialize, Serialize};

use crate::{
    coord,
    utils::{
        coordinate::Coordinate,
        direction::Direction,
        game_error::{GameError, GameResult},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Empty,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileType::Wall => "#",
                TileType::Empty => ".",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub coord: Coordinate,
    pub tile_type: TileType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMap {
    pub tiles: Vec<Tile>,
    pub width: u32,
    pub height: u32,
}

impl GameMap {
    pub fn new(map_path: &Path) -> Self {
        let image = bmp::open(map_path).expect("Failed to read map file");

        let width = image.get_width();
        let height = image.get_height();

        let tiles = image
            .coordinates()
            .map(|(x, y)| {
                let pixel = image.get_pixel(x, y);
                Tile {
                    coord: coord!(x, y),
                    tile_type: Self::tiletype_for_pixel(pixel),
                }
            })
            .collect();

        Self {
            tiles,
            width,
            height,
        }
    }

    fn tiletype_for_pixel(pixel: Pixel) -> TileType {
        match (pixel.r, pixel.g, pixel.b) {
            (0, 0, 0) => TileType::Wall,
            _ => TileType::Empty,
        }
    }

    pub fn get_tile_by_coord(&self, coord: &Coordinate) -> GameResult<Tile> {
        return self.get_tile(coord.x, coord.y);
    }

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

    pub fn get_tile_translated(&self, coord: &Coordinate, dir: &Direction) -> GameResult<Tile> {
        let translated_cord = match dir {
            Direction::Up => coord!(coord.x, coord.y - 1),
            Direction::Down => coord!(coord.x, coord.y + 1),
            Direction::Right => coord!(coord.x + 1, coord.y),
            Direction::Left => coord!(coord.x - 1, coord.y),
        };

        self.get_tile_by_coord(&translated_cord)
    }

    pub fn get_free_tiles(&self) -> Vec<Tile> {
        self.tiles
            .iter()
            .filter(|&tile| tile.tile_type == TileType::Empty)
            .map(|t| t.clone())
            .collect()
    }
}
