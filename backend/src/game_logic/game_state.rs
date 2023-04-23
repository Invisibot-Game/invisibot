use std::collections::HashMap;

use crate::{
    player::{create_players, Player, PlayerId},
    utils::{coordinate::Coordinate, game_error::GameResult},
};

use super::{
    game_map::{GameMap, TileType},
    player::PlayerClients,
};

#[derive(Debug, Clone)]
pub struct GameState {
    pub map: GameMap,
    pub players: HashMap<PlayerId, Player>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            map: GameMap::new(24, 24),
            players: create_players(),
        }
    }

    pub fn run_round(&self, player_clients: &mut PlayerClients) -> GameResult<Self> {
        let requested_destinations = self.get_requested_destinations(player_clients)?;

        let tile_collisions = self.check_collisions(&requested_destinations);

        let next_round_players = self.next_round_players(&requested_destinations, &tile_collisions);

        Ok(Self {
            map: self.map.clone(),
            players: next_round_players,
        })
    }

    fn get_requested_destinations(
        &self,
        player_clients: &mut PlayerClients,
    ) -> GameResult<HashMap<PlayerId, Coordinate>> {
        self.players
            .iter()
            .map(|(id, p)| (id, p.get_pos()))
            .map(|(id, coord)| {
                let dir = player_clients.play_round(&self, id);
                let requested_coord = self
                    .map
                    .get_tile_translated(coord, &dir)
                    .map(|tile| tile.coord)
                    .unwrap_or(coord.clone());
                Ok((id.clone(), requested_coord))
            })
            .collect()
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
                    if let Some(initial_move) = self.get_initial_move(
                        player.get_pos(),
                        id,
                        requested_destinations,
                        tile_collisions,
                    ) {
                        acc.insert(id.clone(), Player::update_pos(player, initial_move, false));
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
            error!("Player count missmatch between round! Prev {prev_players}, new {new_players}");
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
    ) -> Option<Coordinate> {
        let req = if let Some(tile) = requested.get(player_id) {
            tile
        } else {
            // No new destination was requested? Just stay in place
            return Some(curr.clone());
        };

        if !self.is_pos_walkable(req) || self.pos_contains_player(req) {
            // Either a wall or a player is blocking us from going there.
            return Some(curr.clone());
        }

        match collisions.get(req) {
            None => {
                warn!("Coordinate was not in collision map {req:?}\n\nMAP: {collisions:?}");
                Some(curr.clone())
            }
            Some(1) => Some(req.clone()),
            Some(_) => None, // Deal with collisions later
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
