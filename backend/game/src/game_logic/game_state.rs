use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use invisibot_client_api::round_response::RoundResponse;
use invisibot_common::{
    coordinate::Coordinate,
    direction::Direction,
    game_error::{GameError, GameResult},
    player_id::PlayerId,
};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::game_map::{game_map::GameMap, player::Player};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub map: GameMap,
    pub players: HashMap<PlayerId, Player>,
    pub shot_tiles: HashSet<Coordinate>,
}

impl GameState {
    pub fn new(player_ids: HashSet<PlayerId>, map_dir: &str) -> GameResult<Self> {
        let map = Path::new(map_dir);
        let map = GameMap::load_from_image(map)?;
        let players = create_players(&map, player_ids)?;

        Ok(Self {
            map,
            players,
            shot_tiles: HashSet::new(),
        })
    }

    pub fn run_round(
        &self,
        actions: HashMap<PlayerId, RoundResponse>,
    ) -> GameResult<(Self, HashSet<PlayerId>)> {
        let shooting = Self::get_shooting(&actions);
        let rotating = Self::get_rotating(&actions);
        let moving = self.get_moving(&actions)?;

        let mut next_round_players = HashMap::new();
        self.insert_shooting(&mut next_round_players, &shooting)?;
        self.insert_rotating(&mut next_round_players, rotating)?;
        self.insert_moving(&mut next_round_players, moving)?;

        let (shot_tiles, dead_players) = self.handle_shots(&mut next_round_players, shooting)?;

        Ok((
            Self {
                map: self.map.clone(),
                players: next_round_players,
                shot_tiles,
            },
            dead_players,
        ))
    }

    fn get_shooting(actions: &HashMap<PlayerId, RoundResponse>) -> HashSet<PlayerId> {
        actions
            .iter()
            .filter(|&(_, resp)| resp == &RoundResponse::Shoot)
            .map(|(p, _)| p.clone())
            .collect()
    }

    fn get_rotating(actions: &HashMap<PlayerId, RoundResponse>) -> HashMap<PlayerId, Direction> {
        actions
            .iter()
            .filter_map(|(player_id, resp)| match resp {
                RoundResponse::Rotate(dir) => Some((player_id.clone(), dir.clone())),
                _ => None,
            })
            .collect()
    }

    fn get_moving(
        &self,
        actions: &HashMap<PlayerId, RoundResponse>,
    ) -> GameResult<HashMap<PlayerId, Coordinate>> {
        actions
            .iter()
            .filter_map(|(player_id, resp)| match resp {
                RoundResponse::Move(dir) => Some((player_id.clone(), dir.clone())),
                _ => None,
            })
            .map(|(player_id, dir)| {
                let player = self.get_player(&player_id)?;
                let new_tile = self.map.get_tile_translated(player.get_pos(), &dir)?;
                Ok((player_id.clone(), new_tile.coord))
            })
            .collect()
    }

    fn insert_shooting(
        &self,
        next_round_players: &mut HashMap<PlayerId, Player>,
        shooting: &HashSet<PlayerId>,
    ) -> GameResult<()> {
        for id in shooting.iter() {
            let player = self.get_player(id)?;
            let updated_player = player.shoot();
            next_round_players.insert(id.clone(), updated_player);
        }

        Ok(())
    }

    fn insert_rotating(
        &self,
        next_round_players: &mut HashMap<PlayerId, Player>,
        rotating: HashMap<PlayerId, Direction>,
    ) -> GameResult<()> {
        for (id, dir) in rotating.into_iter() {
            let player = self.get_player(&id)?;
            let updated_player = player.rotate(dir);
            next_round_players.insert(id, updated_player);
        }

        Ok(())
    }

    fn insert_moving(
        &self,
        next_round_players: &mut HashMap<PlayerId, Player>,
        moving: HashMap<PlayerId, Coordinate>,
    ) -> GameResult<()> {
        let mut unhandled_moves: HashMap<PlayerId, Coordinate> = HashMap::new();

        // Insert the invalid moves
        for (id, requested_dest) in moving.iter() {
            if !self.map.is_pos_walkable(requested_dest)
                || pos_contains_player(&next_round_players, requested_dest)
            {
                let player = self.get_player(id)?;
                next_round_players.insert(id.clone(), player.clone());
            } else {
                unhandled_moves.insert(id.clone(), requested_dest.clone());
            }
        }

        // Check for collisions before allowing any moves
        let collisions = self.check_collisions(&unhandled_moves, next_round_players)?;

        for (id, requested_dest) in unhandled_moves.into_iter() {
            let collision = collisions
                .get(&requested_dest)
                .ok_or(GameError::InvalidGameState(format!(
                    "Player not in collisions map!"
                )))?
                .clone();

            let player = self.get_player(&id)?;

            match collision {
                0 => {
                    return Err(GameError::InvalidGameState(format!(
                        "0 collisions when at least 1 player wants to move there?"
                    )));
                }
                1 => {
                    let updated_player = player.update_pos(requested_dest, false);
                    // The only player who wants to go there and nobody is currently there.
                    next_round_players.insert(id.clone(), updated_player);
                }
                _ => {
                    // Had collision
                    let updated_player = player.update_visibility(true);
                    next_round_players.insert(id, updated_player);
                }
            }
        }

        Ok(())
    }

    fn check_collisions(
        &self,
        requested_destinations: &HashMap<PlayerId, Coordinate>,
        next_round_players: &HashMap<PlayerId, Player>,
    ) -> GameResult<HashMap<Coordinate, usize>> {
        let mut possible_collisions: Vec<Coordinate> = Vec::new();

        for (id, player) in self.players.iter() {
            if let Some(coord) = requested_destinations.get(id) {
                // Insert their requested coordinate
                possible_collisions.push(coord.clone());

                // Also insert their current position in case their move fails.
                possible_collisions.push(player.get_pos().clone());
            } else {
                let handled_coord = next_round_players
                    .get(id)
                    .ok_or(GameError::InvalidGameState(format!(
                        "Player neither in requested directions nor in next_round_players {id}"
                    )))?
                    .get_pos();

                // The player is already handled, insert their final coordinate.
                possible_collisions.push(handled_coord.clone());
            };
        }

        Ok(possible_collisions
            .into_iter()
            .fold(HashMap::new(), |mut acc, coord| {
                let new_count = if let Some(count) = acc.get(&coord) {
                    count + 1
                } else {
                    1 as usize
                };

                acc.insert(coord, new_count);
                acc
            }))
    }

    fn get_player(&self, player_id: &PlayerId) -> GameResult<&Player> {
        Ok(self
            .players
            .get(player_id)
            .ok_or(GameError::InvalidGameState(format!(
                "Player with id {player_id} did not exist?"
            )))?)
    }

    fn handle_shots(
        &self,
        next_round_players: &mut HashMap<PlayerId, Player>,
        shooting_players: HashSet<PlayerId>,
    ) -> GameResult<(HashSet<Coordinate>, HashSet<PlayerId>)> {
        let mut kill_on_tiles = HashSet::new();

        for player_id in shooting_players.into_iter() {
            let player = next_round_players
                .get(&player_id)
                .ok_or(GameError::InvalidGameState(format!(
                    "Shooting player not in next rounds players?"
                )))?;

            self.map
                .get_line_of_sight(player.get_pos(), player.get_rotation())
                .into_iter()
                .for_each(|tile| {
                    kill_on_tiles.insert(tile);
                });
        }

        let players_to_delete: HashSet<PlayerId> = next_round_players
            .iter()
            .filter(|(_, player)| kill_on_tiles.contains(player.get_pos()))
            .map(|(id, _)| id.clone())
            .collect();

        players_to_delete.iter().for_each(|id| {
            next_round_players.remove(&id);
        });

        Ok((kill_on_tiles, players_to_delete))
    }
}

#[inline(always)]
fn pos_contains_player(map: &HashMap<PlayerId, Player>, pos: &Coordinate) -> bool {
    match get_player_by_coord(map, pos) {
        Some(_) => true,
        None => false,
    }
}

#[inline(always)]
fn get_player_by_coord(map: &HashMap<PlayerId, Player>, pos: &Coordinate) -> Option<Player> {
    if let Some(&p) = map
        .iter()
        .map(|(_, p)| p)
        .filter(|&p| p.get_pos().x == pos.x && p.get_pos().y == pos.y)
        .collect::<Vec<&Player>>()
        .first()
    {
        let p = p.clone();
        Some(p)
    } else {
        None
    }
}

fn create_players(
    map: &GameMap,
    player_ids: HashSet<PlayerId>,
) -> GameResult<HashMap<PlayerId, Player>> {
    let mut starting_positions = map.get_starting_positions();

    if starting_positions.len() < player_ids.len() {
        panic!(
            "This map only supports {} players but {} players have connected!",
            starting_positions.len(),
            player_ids.len()
        );
    }

    starting_positions.shuffle(&mut thread_rng());

    player_ids
        .into_iter()
        .enumerate()
        .map(|(index, id)| {
            Ok((
                id.clone(),
                Player::new(
                    id,
                    starting_positions
                        .get(index)
                        .ok_or(GameError::NotEnoughStartingPositions)?
                        .clone(),
                    false,
                ),
            ))
        })
        .collect()
}
