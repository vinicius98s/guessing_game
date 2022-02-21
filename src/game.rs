use crate::config::Config;
use colored::*;
use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering,
    io::{self, Write},
    ops::RangeInclusive,
};

pub const GAME_RANGE: RangeInclusive<u8> = 1..=10;

pub struct Game {
    retries: i32,
    pub generated_value: u8,
    pub retries_left: i32,
    pub has_ended: bool,
}

impl Game {
    pub fn new(config: &Config) -> Self {
        let generated_value = generate_value(config);

        Game {
            generated_value,
            retries: config.retries,
            retries_left: config.retries,
            has_ended: false,
        }
    }

    pub fn make_guess(game: &mut Self, guess: u8) -> &mut Self {
        output_guess(guess, game.generated_value);
        game.retries_left -= 1;

        let has_ended = game.generated_value == guess || game.retries_left == 0;

        if has_ended {
            game.has_ended = true;
            game
        } else {
            println!("You have {} retries left.\n", game.retries_left);
            game
        }
    }

    pub fn restart<'t>(game: &'t mut Self, config: &Config) -> &'t mut Self {
        game.retries_left = game.retries;
        game.generated_value = generate_value(config);
        game.has_ended = false;

        game
    }
}

/// Generates a random value for the game.
fn generate_value(config: &Config) -> u8 {
    let mut rng = thread_rng();
    let value = rng.gen_range(GAME_RANGE);
    if (*config).debug {
        println!("{} Correct value: {}\n", "[Debug]".cyan(), value);
    }
    value
}

/// It takes the user input and cast the value to u8.
pub fn get_guess() -> u8 {
    loop {
        print!("Input guess: ");
        std::io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not get guess value");

        match guess.trim().parse::<u8>() {
            Ok(value) => return value,
            Err(error) => println!("Could not convert input: {}", error),
        };
    }
}

/// Checks whether the user wants to restart the game or not.
pub fn check_restart_game() -> bool {
    print!(
        "\n{} Do you want to restart? [Y/n]: ",
        "The game has ended!".magenta()
    );
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Could not read end game response");

    match response.trim().to_lowercase().as_str() {
        "y" | "" => true,
        "n" => false,
        _ => false,
    }
}

/// Checks whether the user guess is correct or not.
/// Also prints if is too low or too high.
fn output_guess(guess: u8, correct: u8) {
    match guess.cmp(&correct) {
        Ordering::Equal => println!("{}", "\nYou got it. Congrats!".green()),
        Ordering::Less => println!("{}", "\nToo low!".yellow()),
        Ordering::Greater => println!("{}", "\nToo high!".yellow()),
    }
}
