use std::{
    io::{self, Write},
    process::Command,
};

use hangman::{
    game::{GameResult, GuessErr, State}, secret,
};

fn main() {
    print_initial_message();

    let secret = secret::get_random_word();
    let mut state = State::new(secret);
    let mut game_result = state.is_game_over();

    while game_result == GameResult::OnGoing {
        clear_console();
        print_game_state(&state);
        let guess = get_guess();
        let result = state.try_guess(guess);
        match result {
            Ok(_) => println!("Nice one! {guess} belongs to the mysterious word! 🎯"),
            Err(e) => match e {
                GuessErr::Repeated(msg) => println!("{}", msg),
                GuessErr::Invalid(msg) => println!("{}", msg),
                GuessErr::Incorrect(msg) => println!("{}", msg),
            },
        }
        
        game_result = state.is_game_over();
        pause_console();
    }

    match game_result {
        GameResult::Win => println!("⭐ YOU WON ⭐\nThere were still {} guesses left!", state.guesses_left()),
        GameResult::Lose => println!("YOU LOSE ☹️\nThe secret word was: {}", state.secret()),
        GameResult::OnGoing => panic!("Error: Game loop should never end if game still on going."),
    }

    //TODO: Offers to play again
}

fn get_guess() -> char {
    let mut buffer = String::new();

    loop {
        print!("Enter your guess: ");

        io::stdout().flush().unwrap();
        buffer.clear();

        if io::stdin().read_line(&mut buffer).is_err() {
            println!("Sorry, that option is invalid. Please try again.");
            continue;
        }

        match buffer.trim().parse::<char>() {
            Ok(guess) => return guess,
            Err(_) => {
                println!("Sorry, that option is invalid. Please try again.");
            }
        }
    }
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

    pause_console();

    clear_console();
}

fn pause_console() {
    let mut stdout = io::stdout();
    println!("Press any key to continue...");
    stdout.flush().unwrap();

    let mut _buffer = String::new();
    io::stdin().read_line(&mut _buffer).unwrap();
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/c", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}
