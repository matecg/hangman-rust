pub mod secret {
    use rand::seq::IndexedRandom;

    const SECRETS: [&'static str; 9] = [
        "mystery",
        "broccoli",
        "account",
        "almost",
        "spaghetti",
        "opinion",
        "beautiful",
        "distance",
        "luggage",
    ];

    pub fn get_random_word() -> &'static str {
        let mut rng = rand::rng();

        *SECRETS.choose(&mut rng).unwrap()
    }

    pub fn get_rand_word_from_file(_filename: &str) -> &str {
        todo!()
    }
}

pub mod game {
    #[derive(Debug, PartialEq)]
    pub enum GameResult {
        Win,
        Lose,
        OnGoing,
    }

    #[derive(Debug)]
    pub enum GuessErr {
        Repeated(String),
        Invalid(String),
        Incorrect(String),
    }

    #[derive(Debug)]
    pub struct State {
        guesses_left: u8,
        secret: String,
        guesses: Vec<char>,
        correct_guesses: usize,
    }

    impl State {
        pub fn new(secret: &str) -> Self {
            Self {
                guesses_left: 7,
                secret: String::from(secret.to_ascii_lowercase()),
                guesses: Vec::new(),
                correct_guesses: 0,
            }
        }

        pub fn guesses_left(&self) -> u8 {
            self.guesses_left
        }

        pub fn secret(&self) -> &str {
            &self.secret
        }

        pub fn guesses(&self) -> &Vec<char> {
            &self.guesses
        }

        pub fn try_guess(&mut self, guess: char) -> Result<(), GuessErr> {
            let guess = guess.to_ascii_uppercase();
            if self.guesses_left == 0 {
                return Err(GuessErr::Invalid(format!(
                    "Attempted to guess after running out of guesses."
                )));
            }

            if self.guessed_before(guess) {
                return Err(GuessErr::Repeated(format!("{guess} was already guessed.")));
            }

            if !guess.is_alphabetic() {
                return Err(GuessErr::Invalid(format!("{guess} is not alphabetic.")));
            }

            self.guesses.push(guess.to_ascii_uppercase());
            let matches = self.count_guess_on_secret(guess);
            if matches == 0{
                self.guesses_left -= 1;
                Err(GuessErr::Incorrect(format!(
                    "{guess} does not belong to the mysterious word."
                )))
            } else {
                // Update correct guesses considering all possible repetitions
                self.correct_guesses += matches;
                Ok(())
            }
        }

        pub fn is_game_over(&self) -> GameResult {
            if self.guesses_left == 0 {
                GameResult::Lose
            } else if self.secret_found() {
                GameResult::Win
            } else {
                GameResult::OnGoing
            }
        }

        pub fn guessed_before(&self, guess: char) -> bool {
            let guess = guess.to_ascii_uppercase();
            self.guesses.contains(&guess)
        }

        pub fn count_guess_on_secret(&self, guess: char) -> usize {
            let guess = guess.to_ascii_lowercase();
            self.secret.matches(guess).count()
        }

        fn secret_found(&self) -> bool {
            self.correct_guesses == self.secret.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn can_create_new_game_state() {
            let secret = "secret";
            let actual = State::new(secret);

            assert_eq!(actual.guesses_left, 7);
            assert_eq!(actual.guesses.len(), 0);
            assert_eq!(actual.secret, secret);
        }

        #[test]
        fn detect_repeated_guesses() {
            let guess = 'G';
            let mut state = get_empty_state("secret");

            let actual = state.guessed_before(guess);
            assert!(!actual);

            state.guesses.push(guess);
            let actual = state.guessed_before(guess);
            assert!(actual);
        }

        #[test]
        fn detect_valid_guesses_case_insensitive() {
            let guess = 'S';
            let state = get_empty_state("secret");

            assert_eq!(state.count_guess_on_secret(guess), 1);

            let guess = 's';
            assert_eq!(state.count_guess_on_secret(guess), 1);
        }

        #[test]
        fn detect_invalid_guesses_case_insensitive() {
            let guess = 'K';
            let state = get_empty_state("secret");

            assert_eq!(state.count_guess_on_secret(guess), 0);

            let guess = 'k';
            assert_eq!(state.count_guess_on_secret(guess), 0);
        }

        mod state_try_guess {
            use super::*;

            #[test]
            fn error_when_guess_is_not_alphabetic() {
                const INVALIDS: [char; 5] = ['1', '.', ' ', ';', '0'];
                let mut state = get_empty_state("secret");

                for invalid in INVALIDS {
                    let result = state.try_guess(invalid);

                    if let Err(GuessErr::Invalid(msg)) = result {
                        assert_eq!(msg, format!("{invalid} is not alphabetic."));
                    } else {
                        panic!("Expected GuessErr::Invalid, but got: {:?}", result);
                    }
                }
            }

            #[test]
            fn error_when_guess_is_repeated() {
                let repeated = 'G';
                let mut state = get_empty_state("secret");

                state.guesses.push(repeated);
                let result = state.try_guess(repeated);
                if let Err(GuessErr::Repeated(msg)) = result {
                    assert_eq!(msg, format!("{repeated} was already guessed."));
                } else {
                    panic!("Expected GuessErr::Repeated, but got: {:?}", result);
                }
            }

            #[test]
            fn error_when_there_are_not_more_guesses() {
                let mut state = get_empty_state("secret");
                state.guesses_left = 0;

                let result = state.try_guess('G');
                if let Err(GuessErr::Invalid(msg)) = result {
                    assert_eq!(msg, "Attempted to guess after running out of guesses.");
                } else {
                    panic!("Expected GuessErr::Invalid, but got: {:?}", result);
                }
            }

            #[test]
            fn can_process_incorrect_guess() {
                let invalid = 'g';
                let mut state = get_empty_state("secret");
                let guesses_left = state.guesses_left;

                let result = state.try_guess(invalid);
                if let Err(GuessErr::Incorrect(msg)) = result {
                    assert_eq!(state.guesses_left, guesses_left - 1);
                    assert!(state.guesses.contains(&invalid.to_ascii_uppercase()));
                    assert_eq!(msg, format!("{} does not belong to the mysterious word.", invalid.to_ascii_uppercase()))
                } else {
                    panic!(
                        "Expected to correctly process invalid guess when there are guesses left."
                    );
                }
            }

            #[test]
            fn can_process_valid_guess() {
                let guess = 'S';
                let mut state = get_empty_state("secret");
                let guesses_left = state.guesses_left;

                let result = state.try_guess(guess);
                if let Ok(_) = result {
                    assert_eq!(state.guesses_left, guesses_left);
                    assert!(state.guesses.contains(&guess));
                } else {
                    panic!("Expected to correctly register a valid case insensitive guess.");
                }
            }
        }

        fn get_empty_state(secret: &str) -> State {
            State {
                guesses_left: 7,
                secret: String::from(secret),
                guesses: Vec::new(),
                correct_guesses: 0,
            }
        }
    }
}
