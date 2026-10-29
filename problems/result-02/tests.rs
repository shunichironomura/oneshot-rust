use std::num::ParseIntError;

use crate::judge::{case, check_eq};
use crate::user::{double_first, parse_all, split_parsed, sum_all};

fn err(s: &str) -> ParseIntError {
    s.parse::<i32>().unwrap_err()
}

pub fn run() {
    case("example-1", || {
        check_eq(parse_all(&["7", "-3"]), Ok(vec![7, -3]))?;
        check_eq(parse_all(&["1", "x", ""]), Err(err("x")))
    });
    case("example-2", || {
        check_eq(split_parsed(&["4", "a", "-2", "1.5"]), (vec![4, -2], vec!["a", "1.5"]))?;
        check_eq(double_first(&["21", "x"]), Ok(Some(42)))
    });
    case("example-3", || {
        check_eq(sum_all(&[Some(1), Some(2)]), Some(3))?;
        check_eq(sum_all(&[Some(u32::MAX), Some(1)]), None)
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
    case("case-3", || {
        // All valid, all invalid, nothing at all; the strings outlive the slice that held them.
        check_eq(split_parsed(&[]), (Vec::<i32>::new(), Vec::<&str>::new()))?;
        check_eq(split_parsed(&["1", "+2", "-3"]), (vec![1, 2, -3], Vec::<&str>::new()))?;
        check_eq(
            split_parsed(&["", " 1", "2147483648", "１", "-"]),
            (Vec::<i32>::new(), vec!["", " 1", "2147483648", "１", "-"]),
        )?;
        let text = String::from("10 x 20 7e3 -0");
        let (values, invalid) = {
            let items: Vec<&str> = text.split(' ').collect();
            split_parsed(&items)
        };
        check_eq(values, vec![10, 20, 0])?;
        check_eq(invalid, vec!["x", "7e3"])
    });
    case("case-4", || {
        // Only the first item matters.
        check_eq(double_first(&[]), Ok(None))?;
        check_eq(double_first(&["x", "3"]), Err(err("x")))?;
        check_eq(double_first(&["", "3"]), Err(err("")))?;
        check_eq(double_first(&[" 4"]), Err(err(" 4")))?;
        check_eq(double_first(&["0", "", "x"]), Ok(Some(0)))?;
        check_eq(double_first(&["-8"]), Ok(Some(-16)))
    });
    case("case-5", || {
        // Twice an i32 does not always fit in an i32, but the first item is still an i32.
        check_eq(double_first(&["2147483647"]), Ok(Some(4_294_967_294)))?;
        check_eq(double_first(&["-2147483648", "y"]), Ok(Some(-4_294_967_296)))?;
        check_eq(double_first(&["2147483648"]), Err(err("2147483648")))
    });
    case("case-6", || {
        // Any None gives None; an empty slice sums to 0.
        check_eq(sum_all(&[]), Some(0))?;
        check_eq(sum_all(&[None]), None)?;
        check_eq(sum_all(&[Some(1), None]), None)?;
        check_eq(sum_all(&[None, Some(4), Some(5)]), None)?;
        check_eq(sum_all(&[Some(0), Some(7), Some(0)]), Some(7))
    });
    case("case-7", || {
        // The sum may reach u32::MAX but not go past it.
        check_eq(sum_all(&[Some(u32::MAX)]), Some(u32::MAX))?;
        check_eq(sum_all(&[Some(u32::MAX - 1), Some(1)]), Some(u32::MAX))?;
        check_eq(sum_all(&[Some(3_000_000_000), Some(1_000_000_000), Some(294_967_296)]), None)?;
        check_eq(sum_all(&[Some(2_000_000_000), Some(2_000_000_000), Some(2_000_000_000)]), None)?;
        check_eq(sum_all(&[Some(u32::MAX), Some(u32::MAX), None]), None)
    });
}
