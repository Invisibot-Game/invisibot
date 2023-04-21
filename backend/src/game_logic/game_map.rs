use std::fmt::Display;

use serde::Serialize;

use crate::utils::{
    coordinate::Coordinate,
    game_error::{GameError, GameResult},
};

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Debug, Clone)]
pub struct GameMap {
    pub tiles: Vec<Tile>,
    pub width: u32,
    pub height: u32,
}

impl GameMap {
    pub fn new(width: u32, height: u32) -> Self {
        let tiles = (0..height)
            .map(|y| {
                (0..width).map(move |x| Tile {
                    tile_type: if x == 0 || y == 0 || x == (width - 1) || y == (height - 1) {
                        TileType::Wall
                    } else {
                        TileType::Empty
                    },
                })
            })
            .flatten()
            .collect();

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tile_coord(&self, coord: &Coordinate) -> GameResult<Tile> {
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
}
