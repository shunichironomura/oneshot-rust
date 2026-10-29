use crate::judge::{case, check_eq};
use crate::user::sum_csv;
use std::num::ParseIntError;

fn err(s: &str) -> ParseIntError {
    s.parse::<i64>().unwrap_err()
}

pub fn run() {
    case("example-1", || check_eq(sum_csv("1, 2, 3"), Ok(6)));
    case("example-2", || check_eq(sum_csv("4,x,5"), Err(err("x"))));
    case("case-1", || check_eq(sum_csv(""), Ok(0)));
    case("case-2", || check_eq(sum_csv("  \t "), Ok(0)));
    case("case-3", || check_eq(sum_csv(" -5 ,+3,10 "), Ok(8)));
    case("case-4", || check_eq(sum_csv("1,,2"), Err(err(""))));
    case("case-5", || {
        check_eq(sum_csv("1, 2a, 99999999999999999999"), Err(err("2a")))?;
        check_eq(sum_csv("7, 99999999999999999999, b"), Err(err("99999999999999999999")))
    });
    case("case-6", || check_eq(sum_csv("9223372036854775807"), Ok(i64::MAX)));
}
