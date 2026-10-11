use std::cmp::Ordering;
use std::panic::{self, AssertUnwindSafe};

use crate::judge::{case, check_eq};
use crate::user::{Guess, GuessError, parse_guess};

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

/// `parse_guess(input)` with the guess replaced by its value.
fn parsed(input: &str) -> Result<i32, String> {
    parse_guess(input).map(|g| g.value())
}

fn err(message: &str) -> Result<i32, String> {
    Err(message.to_string())
}

pub fn run() {
    case("sample-1", || {
        check_eq(Guess::new(42).value(), 42)?;
        check_eq(Guess::new(30).compare(50), Ordering::Less)?;
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
    case("sample-3", || {
        check_eq(parsed(" 42 "), Ok(42))?;
        check_eq(parsed("+7"), Ok(7))?;
        check_eq(parsed("0"), err("out of range"))?;
        check_eq(parsed("abc"), err("not a number"))
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
    case("hidden-4", || {
        let g = Guess::new(50);
        check_eq(g.compare(50), Ordering::Equal)?;
        check_eq(g.compare(49), Ordering::Greater)?;
        check_eq(g.compare(51), Ordering::Less)?;
        check_eq(Guess::new(100).compare(1), Ordering::Greater)?;
        check_eq(Guess::new(1).compare(100), Ordering::Less)?;
        // The secret may lie outside 1 to 100.
        check_eq(Guess::new(7).compare(-3), Ordering::Greater)?;
        check_eq(Guess::new(100).compare(1000), Ordering::Less)?;
        check_eq(Guess::new(1).compare(i32::MIN), Ordering::Greater)?;
        check_eq(Guess::new(100).compare(i32::MAX), Ordering::Less)
    });
    case("hidden-5", || {
        // Any whitespace at both ends is removed, as str::trim does.
        check_eq(parsed("\t42\n"), Ok(42))?;
        check_eq(parsed("100\r\n"), Ok(100))?;
        check_eq(parsed("\u{3000}55\u{3000}"), Ok(55))?;
        check_eq(parsed("1"), Ok(1))?;
        check_eq(parsed("+100"), Ok(100))?;
        check_eq(parsed("007"), Ok(7))
    });
    case("hidden-6", || {
        // Numbers that i32 accepts but that are not valid guesses.
        check_eq(parsed("-5"), err("out of range"))?;
        check_eq(parsed("101"), err("out of range"))?;
        check_eq(parsed("300"), err("out of range"))?;
        check_eq(parsed(" -0 "), err("out of range"))?;
        check_eq(parsed("2147483647"), err("out of range"))?;
        check_eq(parsed("-2147483648"), err("out of range"))
    });
    case("hidden-7", || {
        // Text that i32 does not accept, including numbers that do not fit in an i32.
        check_eq(parsed(""), err("not a number"))?;
        check_eq(parsed(" \t\n"), err("not a number"))?;
        check_eq(parsed("4 2"), err("not a number"))?;
        check_eq(parsed("4.5"), err("not a number"))?;
        check_eq(parsed("42abc"), err("not a number"))?;
        check_eq(parsed("+-7"), err("not a number"))?;
        check_eq(parsed("+"), err("not a number"))?;
        check_eq(parsed("3000000000"), err("not a number"))?;
        check_eq(parsed("-3000000000"), err("not a number"))
    });
}
