use crate::judge::{case, check_eq};
use crate::user::{BadField, sum_ids, validate_row};

fn bad<T>(column: usize, text: &str) -> Result<T, BadField> {
    Err(BadField { column, text: String::from(text) })
}

pub fn run() {
    case("sample-1", || {
        check_eq(validate_row("7, 30, 1.5"), Ok(()))?;
        check_eq(validate_row("7, 300, 1.5"), bad(1, "300"))?;
        check_eq(validate_row("7"), bad(1, ""))
    });
    case("sample-2", || check_eq(sum_ids(&["7, 30, 1.5", "35, 41, 0.25"]), Ok(42)));
    case("hidden-1", || {
        // The id is a u32: no fractions, no negatives, at most 4294967295.
        check_eq(validate_row("4294967295, 0, 0"), Ok(()))?;
        check_eq(validate_row("1.5, 30, 1"), bad(0, "1.5"))?;
        check_eq(validate_row("-1, 30, 1"), bad(0, "-1"))?;
        check_eq(validate_row("4294967296, 30, 1"), bad(0, "4294967296"))
    });
    case("hidden-2", || {
        // The age is a u8 (0 to 255), the score an f64.
        check_eq(validate_row("1, 255, -0.75"), Ok(()))?;
        check_eq(validate_row("1, 256, 0"), bad(1, "256"))?;
        check_eq(validate_row("1, -1, 0"), bad(1, "-1"))?;
        check_eq(validate_row("1, 2, abc"), bad(2, "abc"))?;
        check_eq(validate_row("1, 2"), bad(2, ""))
    });
    case("hidden-3", || {
        // Columns are checked in order, trimmed, and extra columns are ignored.
        check_eq(validate_row("x, 300, y"), bad(0, "x"))?;
        check_eq(validate_row("1, 300, y"), bad(1, "300"))?;
        check_eq(validate_row(""), bad(0, ""))?;
        check_eq(validate_row(" \t5\t,\t6 ,  7  "), Ok(()))?;
        check_eq(validate_row("1,  30 0 ,2"), bad(1, "30 0"))?;
        check_eq(validate_row("1, 2, 1,5, junk"), Ok(()))
    });
    case("hidden-4", || {
        // The sum may exceed u32::MAX, and the first invalid line decides the error.
        check_eq(sum_ids(&[]), Ok(0))?;
        check_eq(sum_ids(&["4294967295, 1, 1", "4294967295, 2, 2"]), Ok(8589934590))?;
        check_eq(sum_ids(&["1, 2, 3", "4, 5, x", "y, 6, 7"]), bad(2, "x"))?;
        check_eq(sum_ids(&["1, 2, 3", "4, 256, 5"]), bad(1, "256"))
    });
}
