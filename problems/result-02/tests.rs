use std::num::ParseIntError;

use crate::judge::{case, check_eq};
use crate::user::parse_all;

fn err(s: &str) -> ParseIntError {
    s.parse::<i32>().unwrap_err()
}

pub fn run() {
    case("example-1", || {
        check_eq(parse_all(&["7", "-3"]), Ok(vec![7, -3]))?;
        check_eq(parse_all(&["1", "x", ""]), Err(err("x")))
    });
    case("case-1", || {
        // An empty slice, signs and the extremes of i32 are fine; whitespace is not trimmed.
        check_eq(parse_all(&[]), Ok(Vec::new()))?;
        check_eq(
            parse_all(&["+5", "-0", "2147483647", "-2147483648"]),
            Ok(vec![5, 0, i32::MAX, i32::MIN]),
        )?;
        check_eq(parse_all(&["1", " 2"]), Err(err(" 2")))?;
        check_eq(parse_all(&["3 ", "4"]), Err(err("3 ")))
    });
    case("case-2", || {
        // The first invalid item decides the error, even when a later one fails differently.
        check_eq(parse_all(&["1", "", "x"]), Err(err("")))?;
        check_eq(parse_all(&["5", "2147483648", "x"]), Err(err("2147483648")))?;
        check_eq(parse_all(&["-2147483649", "y", ""]), Err(err("-2147483649")))
    });
}
