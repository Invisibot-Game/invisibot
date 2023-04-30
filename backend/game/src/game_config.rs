/// Configuration for a particular game.
#[derive(Debug, Clone)]
pub struct GameConfig {
    /// The number of players to be in the game.
    pub num_players: usize,
    /// The max number of rounds to play before aborting the game.
    pub num_rounds: usize,
    /// The file-location of the map to use.
    pub map_dir: String,
}
