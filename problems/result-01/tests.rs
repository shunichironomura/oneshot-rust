use crate::judge::{case, check_eq};
use crate::user::sum_csv;
use std::num::ParseIntError;

fn err(s: &str) -> ParseIntError {
    s.parse::<i64>().unwrap_err()
}

pub fn run() {
    case("sample-1", || check_eq(sum_csv("1, 2, 3"), Ok(6)));
    case("sample-2", || check_eq(sum_csv("4,x,5"), Err(err("x"))));
    case("hidden-1", || check_eq(sum_csv(""), Ok(0)));
    case("hidden-2", || check_eq(sum_csv("  \t "), Ok(0)));
    case("hidden-3", || check_eq(sum_csv(" -5 ,+3,10 "), Ok(8)));
    case("hidden-4", || check_eq(sum_csv("1,,2"), Err(err(""))));
    case("hidden-5", || {
        check_eq(sum_csv("1, 2a, 99999999999999999999"), Err(err("2a")))?;
        check_eq(sum_csv("7, 99999999999999999999, b"), Err(err("99999999999999999999")))
    });
    case("hidden-6", || check_eq(sum_csv("9223372036854775807"), Ok(i64::MAX)));
}
