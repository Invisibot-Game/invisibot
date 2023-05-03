use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    coord,
    utils::{
        coordinate::Coordinate,
        direction::Direction,
        game_error::{GameError, GameResult},
        tile_type::TileType,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub coord: Coordinate,
    pub tile_type: TileType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMap {
    pub tiles: Vec<Tile>,
    starting_positions: Vec<Coordinate>,
    pub width: u32,
    pub height: u32,
}

impl GameMap {
    pub fn new(map_path: &Path) -> GameResult<Self> {
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

    pub fn get_starting_positions(&self) -> Vec<Coordinate> {
        self.starting_positions.clone()
    }

    pub fn is_pos_walkable(&self, pos: &Coordinate) -> bool {
        let tile = if let Ok(tile) = self.get_tile_by_coord(pos) {
            tile
        } else {
            return false;
        };

        tile.tile_type != TileType::Wall
    }

    pub fn get_line_of_sight(&self, coord: &Coordinate, dir: &Direction) -> Vec<Coordinate> {
        let mut tile = coord.translate(dir);
        let mut tiles = Vec::new();
        while self.is_pos_walkable(&tile) {
            tiles.push(tile.clone());
            tile = tile.translate(dir);
        }

        tiles
    }
}
