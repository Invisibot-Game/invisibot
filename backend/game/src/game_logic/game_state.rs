use std::{collections::HashMap, path::Path};

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{
    player::{Player, PlayerId},
    utils::{coordinate::Coordinate, direction::Direction, game_error::GameResult},
};

use super::game_map::{GameMap, TileType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub map: GameMap,
    pub players: HashMap<PlayerId, Player>,
}

impl GameState {
    pub fn new(player_ids: Vec<PlayerId>) -> Self {
        let map = Path::new("./resources/maps/game_map_2.bmp");
        let map = GameMap::new(map);
        let players = create_players(&map, player_ids);

        Self { map, players }
    }

    pub fn run_round(&self, moves: HashMap<PlayerId, Direction>) -> GameResult<Self> {
        let requested_destinations = moves
            .into_iter()
            .map(|(id, dir)| {
                let player = self.players.get(&id).expect("Player did not exist?");
                let new_tile = self
                    .map
                    .get_tile_translated(player.get_pos(), &dir)
                    .expect("Failed to translate tile");
                (id, new_tile.coord)
            })
            .collect();
        // let requested_destinations = self.get_requested_destinations(player_clients)?;

        let tile_collisions = self.check_collisions(&requested_destinations);

        let next_round_players = self.next_round_players(&requested_destinations, &tile_collisions);

        Ok(Self {
            map: self.map.clone(),
            players: next_round_players,
        })
    }

    fn check_collisions(
        &self,
        requested_destinations: &HashMap<u32, Coordinate>,
    ) -> HashMap<Coordinate, usize> {
        requested_destinations
            .iter()
            .fold(HashMap::new(), |mut acc, (_, coord)| {
                let val = if let Some(count) = acc.get(coord) {
                    count + 1
                } else {
                    1
                };
                acc.insert(coord.clone(), val);
                acc
            })
    }

    /// Figure out the next state of players
    fn next_round_players(
        &self,
        requested_destinations: &HashMap<PlayerId, Coordinate>,
        tile_collisions: &HashMap<Coordinate, usize>,
    ) -> HashMap<PlayerId, Player> {
        // Build a map with all 'simple' moves
        let mut next_round_players =
            self.players
                .iter()
                .fold(HashMap::new(), |mut acc, (id, player)| {
                    if let Some((initial_move, should_be_visible)) = self.get_initial_move(
                        player.get_pos(),
                        id,
                        requested_destinations,
                        tile_collisions,
                    ) {
                        acc.insert(
                            id.clone(),
                            Player::update_pos(player, initial_move, should_be_visible),
                        );
                    }

                    acc
                });

        // Go through the ones who haven't been handled yet
        let unhandled_players: HashMap<PlayerId, Player> = self
            .players
            .iter()
            .filter(|&(id, _)| next_round_players.contains_key(id) == false)
            .map(|(id, player)| (id.clone(), player.clone()))
            .collect();

        unhandled_players.into_iter().for_each(|(id, player)| {
            if let Some(req) = requested_destinations.get(&id) {
                if !pos_contains_player(&next_round_players, req) {
                    // Currently favours the player first in the list, might want to change this to the player who is quickest to respond in the future? or smth
                    next_round_players.insert(id, Player::update_pos(&player, req.clone(), true));
                    return;
                }
            }

            next_round_players.insert(id.clone(), Player::update_visibility(&player, true));
        });

        let prev_players = self.players.len();
        let new_players = next_round_players.len();
        if prev_players != new_players {
            println!("ERROR: Player count missmatch between round! Prev {prev_players}, new {new_players}");
        }
        next_round_players
    }

    /// Get the initial move or None if no simple move is available
    fn get_initial_move(
        &self,
        curr: &Coordinate,
        player_id: &PlayerId,
        requested: &HashMap<PlayerId, Coordinate>,
        collisions: &HashMap<Coordinate, usize>,
    ) -> Option<(Coordinate, bool)> {
        let req = if let Some(tile) = requested.get(player_id) {
            tile
        } else {
            // No new destination was requested? Just stay in place
            return Some((curr.clone(), false));
        };

        if !self.is_pos_walkable(req) || self.pos_contains_player(req) {
            // Either a wall or a player is blocking us from going there.
            return Some((curr.clone(), true));
        }

        match collisions.get(req) {
            None => {
                println!(
                    "WARNING: Coordinate was not in collision map {req:?}\n\nMAP: {collisions:?}"
                );
                Some((curr.clone(), false))
            }
            Some(1) => Some((req.clone(), false)), // Noone else wanted to move here, let's just do it!
            Some(_) => None,                       // Deal with collisions later
        }
    }

    /// Checks if the provided pos is within the bounds of the map and does not contain a wall
    fn is_pos_walkable(&self, pos: &Coordinate) -> bool {
        let tile = if let Ok(tile) = self.map.get_tile_by_coord(pos) {
            tile
        } else {
            return false;
        };

        tile.tile_type != TileType::Wall
    }

    /// Checks if the position currently has a player
    fn pos_contains_player(&self, pos: &Coordinate) -> bool {
        pos_contains_player(&self.players, pos)
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

fn create_players(map: &GameMap, player_ids: Vec<PlayerId>) -> HashMap<PlayerId, Player> {
    let mut free_tiles = map.get_free_tiles();
    free_tiles.shuffle(&mut thread_rng());

    player_ids
        .into_iter()
        .enumerate()
        .map(|(index, id)| {
            (
                id.clone(),
                Player::new(
                    id,
                    free_tiles
                        .get(index)
                        .expect("No more free tiles!")
                        .coord
                        .clone(),
                    false,
                ),
            )
        })
        .collect()
}
