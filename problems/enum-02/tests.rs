use crate::judge::{case, check_eq};
use crate::user::Cell::{Empty, Int};
use crate::user::{Cell, numeric_sum, parse_row, texts};

fn text(s: &str) -> Cell {
    Cell::Text(s.to_string())
}

pub fn run() {
    case("sample-1", || {
        check_eq(
            parse_row("3, +5, 1.5,  blue sky , "),
            vec![Int(3), Int(5), text("1.5"), text("blue sky"), Empty],
        )?;
        check_eq(parse_row(""), vec![Empty])
    });
    case("sample-2", || {
        // The row of sample-1, built directly.
        let row = vec![Int(3), Int(5), text("1.5"), text("blue sky"), Empty];
        check_eq(numeric_sum(&row), 8)?;
        check_eq(texts(&row), vec!["1.5", "blue sky"])
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
        // Text: anything the i64 parser rejects, trimmed but otherwise unchanged.
        check_eq(
            parse_row("1_000,0x10, - 3 ,-,12abc,1e3,  café au lait\t"),
            vec![
                text("1_000"),
                text("0x10"),
                text("- 3"),
                text("-"),
                text("12abc"),
                text("1e3"),
                text("café au lait"),
            ],
        )
    });
    case("hidden-4", || {
        check_eq(numeric_sum(&[]), 0)?;
        check_eq(numeric_sum(&[text("9"), Empty, text("x")]), 0)?;
        check_eq(numeric_sum(&[Int(4), Int(-3), text("9"), Empty, Int(10)]), 11)?;
        check_eq(numeric_sum(&[Int(i64::MAX), Int(i64::MIN)]), -1)
    });
    case("hidden-5", || {
        let row = vec![Empty, text("a"), Int(1), text(""), text("b c"), Int(2), Empty];
        check_eq(texts(&row), vec!["a", "", "b c"])?;
        check_eq(texts(&[]), Vec::<&str>::new())
    });
}
