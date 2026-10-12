use std::panic::{self, AssertUnwindSafe};

use crate::judge::{case, check_eq};
use crate::user::{Guess, GuessError};

/// The message of the panic raised by `Guess::new(value)`, or `None` if it returns normally.
fn panic_message(value: i32) -> Option<String> {
    let payload = panic::catch_unwind(AssertUnwindSafe(|| Guess::new(value))).err()?;
    let message = match payload.downcast_ref::<String>() {
        Some(s) => s.clone(),
        None => match payload.downcast_ref::<&str>() {
            Some(s) => s.to_string(),
            None => "(a panic payload that is not a string)".to_string(),
        },
    };
    Some(message)
}

pub fn run() {
    case("sample-1", || {
        check_eq(Guess::new(42).value(), 42)?;
        check_eq(Guess::try_from(100).map(|g| g.value()), Ok(100))?;
        check_eq(Guess::try_from(0), Err(GuessError::TooSmall(0)))
    });
    case("sample-2", || {
        check_eq(
            panic_message(0).as_deref(),
            Some("Guess value must be greater than or equal to 1, got 0."),
        )?;
        check_eq(
            panic_message(200).as_deref(),
            Some("Guess value must be less than or equal to 100, got 200."),
        )
    });
    case("hidden-1", || {
        // Both ends of the range are valid.
        check_eq(Guess::new(1).value(), 1)?;
        check_eq(Guess::new(100).value(), 100)?;
        check_eq(Guess::new(57).value(), 57)?;
        check_eq(panic_message(1), None)?;
        check_eq(panic_message(100), None)?;
        check_eq(Guess::try_from(1).map(|g| g.value()), Ok(1))?;
        let r: Result<Guess, GuessError> = 55.try_into();
        check_eq(r.map(|g| g.value()), Ok(55))
    });
    case("hidden-2", || {
        // The error holds the value that was given.
        check_eq(Guess::try_from(101), Err(GuessError::TooLarge(101)))?;
        check_eq(Guess::try_from(-7), Err(GuessError::TooSmall(-7)))?;
        check_eq(Guess::try_from(i32::MIN), Err(GuessError::TooSmall(i32::MIN)))?;
        check_eq(Guess::try_from(i32::MAX), Err(GuessError::TooLarge(i32::MAX)))?;
        let r: Result<Guess, GuessError> = 1000.try_into();
        check_eq(r, Err(GuessError::TooLarge(1000)))
    });
    case("hidden-3", || {
        check_eq(
            panic_message(101).as_deref(),
            Some("Guess value must be less than or equal to 100, got 101."),
        )?;
        check_eq(
            panic_message(-5).as_deref(),
            Some("Guess value must be greater than or equal to 1, got -5."),
        )?;
        check_eq(
            panic_message(i32::MIN).as_deref(),
            Some("Guess value must be greater than or equal to 1, got -2147483648."),
        )
    });
}
