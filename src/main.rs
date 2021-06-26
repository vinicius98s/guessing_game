use std::env::args;

mod config;
use config::Config;

mod game;
use game::{check_restart_game, get_guess, Game, GAME_RANGE};

/// The guessing game!
///
/// Available options:
///
/// - debug:
///     Will print the generated value.
///     Usage: `--debug`
///
/// - retries:
///     Defines the number of retries for the game.
///     Usage: `--retries=10`
///
/// # Run example
/// ```rust
/// cargo run -- --debug --retries=10
/// ```
///

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::new(args);
    let mut game = Game::new(&config);

    println!("Welcome to the guessing game!");
    println!("The game is simple, you must guess what value we generated.");
    println!(
        "This value can go from {} to {}.",
        GAME_RANGE.start(),
        GAME_RANGE.end()
    );
    println!("\nYou have {} retries. Good luck!\n", game.retries_left);

    loop {
        if !game.has_ended {
            let guess = get_guess();
            Game::make_guess(&mut game, guess);
        } else {
            let should_restart = check_restart_game();
            if should_restart {
                Game::restart(&mut game, &config);
            } else {
                break;
            }
        }
    }
}
