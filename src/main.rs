use game_state::GameState;

mod game_error;
mod game_map;
mod game_state;
mod player;
mod utils;

fn main() {
    let game_state = GameState::new();
    run_game(game_state)
}

fn run_game(initial_state: GameState) {
    let mut state: GameState = initial_state;
    for round in 0..10 {
        println!("Round {round}");
        state.print_map();
        println!();
        state = state.run_round();
    }

    println!("End of game");
    state.print_map();
}
