use std::fmt::Display;

use crate::{
    game_error::{GameError, GameResult},
    utils::coordinate::Coordinate,
};

#[derive(Debug, Clone)]
pub enum TileType {
    Wall,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Tile {
    tile_type: TileType,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.tile_type {
                TileType::Wall => "#",
                TileType::Empty => ".",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct GameMap {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
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

    pub fn print(&self) {
        println!(
            "MAP:\n{}",
            (0..self.height)
                .map(|y| {
                    (0..self.width)
                        .map(|x| self.get_tile(x, y).expect("Failed to get tile").to_string())
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
