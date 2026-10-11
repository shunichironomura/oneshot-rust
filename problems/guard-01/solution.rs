use std::cmp::Ordering;

/// A guess in the guessing game. The field is private, so a `Guess` can only be created by
/// `new` or `try_from`, and its value is always between 1 and 100.
#[derive(Debug, PartialEq)]
pub struct Guess {
    value: i32,
}

#[derive(Debug, PartialEq)]
pub enum GuessError {
    TooSmall(i32),
    TooLarge(i32),
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        match Guess::try_from(value) {
            Ok(guess) => guess,
            Err(GuessError::TooSmall(value)) => {
                panic!("Guess value must be greater than or equal to 1, got {value}.")
            }
            Err(GuessError::TooLarge(value)) => {
                panic!("Guess value must be less than or equal to 100, got {value}.")
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn compare(&self, secret: i32) -> Ordering {
        self.value.cmp(&secret)
    }
}

impl TryFrom<i32> for Guess {
    type Error = GuessError;

    fn try_from(value: i32) -> Result<Guess, GuessError> {
        if value < 1 {
            Err(GuessError::TooSmall(value))
        } else if value > 100 {
            Err(GuessError::TooLarge(value))
        } else {
            Ok(Guess { value })
        }
    }
}

pub fn parse_guess(input: &str) -> Result<Guess, String> {
    let number: i32 = input
        .trim()
        .parse()
        .map_err(|_| "not a number".to_string())?;
    Guess::try_from(number).map_err(|_| "out of range".to_string())
}
