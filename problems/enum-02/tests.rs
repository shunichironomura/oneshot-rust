use crate::judge::{case, check_eq};
use crate::user::Cell::{Empty, Float, Int};
use crate::user::{Cell, numeric_sum, parse_row, texts};

fn text(s: &str) -> Cell {
    Cell::Text(s.to_string())
}

pub fn run() {
    case("sample-1", || {
        check_eq(
            parse_row("3, +5, 1e3, -0.5,  blue sky , "),
            vec![Int(3), Int(5), Float(1000.0), Float(-0.5), text("blue sky"), Empty],
        )?;
        check_eq(parse_row(""), vec![Empty])
    });
    case("sample-2", || {
        // The row of sample-1, built directly.
        let row = vec![Int(3), Int(5), Float(1000.0), Float(-0.5), text("blue sky"), Empty];
        check_eq(numeric_sum(&row), 1007.5)?;
        check_eq(texts(&row), vec!["blue sky"])
    });
    case("hidden-1", || {
        // Every comma separates two fields; whitespace-only fields (tabs too) are empty.
        check_eq(parse_row("  "), vec![Empty])?;
        check_eq(parse_row("1,,2"), vec![Int(1), Empty, Int(2)])?;
        check_eq(parse_row(","), vec![Empty, Empty])?;
        check_eq(parse_row("\t7\t, \t"), vec![Int(7), Empty])
    });
    case("hidden-2", || {
        // Integers: signs, leading zeros and the whole i64 range.
        check_eq(
            parse_row("-0,007,3000000000,-9223372036854775808, 9223372036854775807"),
            vec![Int(0), Int(7), Int(3_000_000_000), Int(i64::MIN), Int(i64::MAX)],
        )
    });
    case("hidden-3", || {
        // Floats: integers outside i64 fall back to f64, and every form f64 accepts counts.
        check_eq(
            parse_row("9223372036854775808, .5, 5., inf, -2.5E-1"),
            vec![
                Float(9_223_372_036_854_775_808.0),
                Float(0.5),
                Float(5.0),
                Float(f64::INFINITY),
                Float(-0.25),
            ],
        )
    });
    case("hidden-4", || {
        // Text: anything neither parser accepts, trimmed but otherwise unchanged.
        check_eq(
            parse_row("1_000,0x10, - 3 ,-,12abc,  café au lait\t"),
            vec![
                text("1_000"),
                text("0x10"),
                text("- 3"),
                text("-"),
                text("12abc"),
                text("café au lait"),
            ],
        )
    });
    case("hidden-5", || {
        check_eq(numeric_sum(&[]), 0.0)?;
        check_eq(numeric_sum(&[text("9"), Empty, text("x")]), 0.0)?;
        check_eq(numeric_sum(&[Float(0.25), Int(-3), text("9"), Empty, Float(1.5)]), -1.25)?;
        // Each value is converted to f64 first, so this does not overflow i64.
        check_eq(numeric_sum(&[Int(i64::MAX), Int(i64::MAX)]), 18_446_744_073_709_551_616.0)?;
        // 2^53 + 1 becomes 2^53 as f64, and 2^53 + 1.0 rounds to 2^53 again; an exact integer
        // sum (e.g. in i128) would give 2^53 + 2.
        check_eq(numeric_sum(&[Int(9_007_199_254_740_993), Int(1)]), 9_007_199_254_740_992.0)
    });
    case("hidden-6", || {
        let row = vec![Empty, text("a"), Int(1), text(""), text("b c"), Float(2.0), Empty];
        check_eq(texts(&row), vec!["a", "", "b c"])?;
        check_eq(texts(&[]), Vec::<&str>::new())
    });
}
