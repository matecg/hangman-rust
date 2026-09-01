use std::{
    io::{self, Write},
    process::Command,
};

use hangman::{game::State, secret};

fn main() {
    print_initial_message();

    let secret = secret::get_random_word();
    let state = State::new(secret);

    print_game_state(&state);
    // Collect guess
    // Process guess
    // Repeat until game over
    // Offers to play again
}

fn print_game_state(state: &State) {
    print_separator();
    println!();
    for c in state.secret().chars() {
        if state.guessed_before(c) {
            print!(" {} ", c.to_ascii_uppercase())
        } else {
            print!(" _ ");
        }
    }
    println!();
    print_separator();
    println!("\nAttempts left: {}", state.guesses_left());
    print!("Guessed: ");
    for c in state.guesses() {
        if !state.guess_in_word(*c) {
            print!(" {}, ", c.to_ascii_uppercase());
        }
    }
    println!("\n");
    print_separator();
}

fn print_separator() {
    println!("{}", "=".repeat(75));
}

fn print_initial_message() {
    clear_console();
    print_separator();
    println!("\n\t\t\tWELCOME TO RUST HANGMAN\n");
    print_separator();
    println!(
        "The instructions are simple:
\t1. There is a hidden word you must guess to win;
\t2. You have up to 7 guesses;
\t3. Wrong guesses decrease that number, 
\t\tif it reaches zero you lose.
        \n\tBest of luck! 😎"
    );
    print_separator();

    let mut stdout = io::stdout();
    println!("Press any key to continue...");
    stdout.flush().unwrap();

    let mut _buffer = String::new();
    io::stdin().read_line(&mut _buffer).unwrap();

    clear_console();
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/c", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}
