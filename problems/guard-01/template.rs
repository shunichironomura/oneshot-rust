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
        todo!()
    }

    pub fn value(&self) -> i32 {
        todo!()
    }
}

impl TryFrom<i32> for Guess {
    type Error = GuessError;

    fn try_from(value: i32) -> Result<Guess, GuessError> {
        todo!()
    }
}
