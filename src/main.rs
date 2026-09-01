use hangman::{secret, game};

fn main() {
    let secret = secret::get_random_word();
    let state = game::State::new(secret);

    println!("{:#?}", state);
}
