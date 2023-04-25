use crate::{player::PlayerId, utils::direction::Direction};

use super::game_state::GameState;

pub struct PlayerClients {
    player_one: PlayerOne,
    player_two: PlayerTwo,
}

impl PlayerClients {
    pub fn new() -> Self {
        Self {
            player_one: PlayerOne,
            player_two: PlayerTwo,
        }
    }

    pub fn play_round(&mut self, game_state: &GameState, player_id: &PlayerId) -> Direction {
        match player_id {
            0 => self.player_one.play_round(game_state, player_id),
            1 => self.player_two.play_round(game_state, player_id),
            _ => {
                println!("Error: PlayerId out of scope");
                Direction::Down
            }
        }
    }
}

pub trait PlayerClient {
    fn play_round(&mut self, game_state: &GameState, player_id: &PlayerId) -> Direction;
}

pub struct PlayerOne;

impl PlayerClient for PlayerOne {
    fn play_round(&mut self, game_state: &GameState, player_id: &PlayerId) -> Direction {
        let player = game_state.players[player_id].clone();
        let x = player.get_pos().x;
        let y = player.get_pos().y;

        let bottom_edge = game_state.map.height - 2;
        let right_edge = game_state.map.width - 2;

        match (x, y) {
            (1, 1) => Direction::Down,
            (1, _) if y == bottom_edge => Direction::Right,
            (1, _) => Direction::Down,
            (_, 1) => Direction::Left,
            (_, _) if x == right_edge => Direction::Up,
            (_, _) if y == bottom_edge => Direction::Right,
            (_, _) => Direction::Up,
        }
    }
}

pub struct PlayerTwo;

impl PlayerClient for PlayerTwo {
    fn play_round(&mut self, game_state: &GameState, player_id: &PlayerId) -> Direction {
        let player = game_state.players[player_id].clone();
        let x = player.get_pos().x;
        let y = player.get_pos().y;

        let bottom_edge = game_state.map.height - 2;
        let right_edge = game_state.map.width - 2;

        match (x, y) {
            (1, 1) => Direction::Right,
            (1, _) if y == bottom_edge => Direction::Up,
            (1, _) => Direction::Up,
            (_, 1) => Direction::Right,
            (_, _) if x == right_edge => Direction::Down,
            (_, _) if y == bottom_edge => Direction::Left,
            (_, _) => Direction::Down,
        }
    }
}
