use std::error::Error;

use crate::judge::{case, check_eq};
use crate::user::{AppError, parse_pair, sum_pairs};

type Sum = Result<f64, Box<dyn Error>>;

// The messages of the parse errors (their `to_string()`).
const INVALID_DIGIT: &str = "invalid digit found in string";
const EMPTY_INT: &str = "cannot parse integer from empty string";
const TOO_LARGE: &str = "number too large to fit in target type";
const TOO_SMALL: &str = "number too small to fit in target type";
const INVALID_FLOAT: &str = "invalid float literal";
const EMPTY_FLOAT: &str = "cannot parse float from empty string";

fn parse_err(msg: &str) -> AppError {
    AppError::Parse(msg.to_string())
}

/// Expects `Ok(want)`.
fn expect_sum(got: Sum, want: f64) -> Result<(), String> {
    match got {
        Ok(v) => check_eq(v, want),
        Err(e) => Err(format!("expected Ok({want:?}), got Err({e:?})")),
    }
}

/// Expects a boxed `AppError` equal to `want` and displayed as `shown`.
fn expect_err(got: Sum, want: AppError, shown: &str) -> Result<(), String> {
    match got {
        Ok(v) => Err(format!("expected Err({want:?}), got Ok({v:?})")),
        Err(e) => match e.downcast_ref::<AppError>() {
            Some(inner) => {
                check_eq(inner, &want)?;
                check_eq(e.to_string(), shown)
            }
            None => Err(format!("expected Err({want:?}), got Err({e:?}), not an AppError")),
        },
    }
}

pub fn run() {
    case("sample-1", || {
        check_eq(parse_pair("3, 0.5"), Ok((3, 0.5)))?;
        check_eq(parse_pair("x,1"), Err(parse_err(INVALID_DIGIT)))?;
        check_eq(parse_pair("1,abc"), Err(parse_err(INVALID_FLOAT)))
    });
    case("sample-2", || expect_sum(sum_pairs("1,0.5\n\n2,1.5"), 5.0));
    case("sample-3", || expect_err(sum_pairs("1,1\n2"), AppError::MissingComma, "missing comma"));
    case("hidden-1", || {
        // No comma at all, and empty parts.
        check_eq(parse_pair("1"), Err(AppError::MissingComma))?;
        check_eq(parse_pair(""), Err(AppError::MissingComma))?;
        check_eq(parse_pair("1;2.5"), Err(AppError::MissingComma))?;
        check_eq(parse_pair(","), Err(parse_err(EMPTY_INT)))?;
        check_eq(parse_pair("1,"), Err(parse_err(EMPTY_FLOAT)))
    });
    case("hidden-2", || {
        // Split at the first comma; both parts are trimmed.
        check_eq(parse_pair("1,2,3"), Err(parse_err(INVALID_FLOAT)))?;
        check_eq(parse_pair(" 7 , 2.5 "), Ok((7, 2.5)))?;
        check_eq(parse_pair("\t-7,\t1e3 "), Ok((-7, 1000.0)))?;
        check_eq(parse_pair("+4,.25"), Ok((4, 0.25)))
    });
    case("hidden-3", || {
        // `a` is checked first, and it must fit in an i32.
        check_eq(parse_pair("x,y"), Err(parse_err(INVALID_DIGIT)))?;
        check_eq(parse_pair("1.5,2"), Err(parse_err(INVALID_DIGIT)))?;
        check_eq(parse_pair("2147483648,0"), Err(parse_err(TOO_LARGE)))?;
        check_eq(parse_pair("-2147483649,0"), Err(parse_err(TOO_SMALL)))?;
        check_eq(parse_pair("-2147483648,0"), Ok((i32::MIN, 0.0)))
    });
    case("hidden-4", || {
        // Blank lines are skipped, so no pairs at all sum to 0.0.
        expect_sum(sum_pairs(""), 0.0)?;
        expect_sum(sum_pairs("  \n\t\n"), 0.0)?;
        expect_sum(sum_pairs("1,1\r\n2,2\r\n"), 6.0)?;
        expect_sum(sum_pairs("-3, 0.25\n   \n 4 ,-1.5\n"), -0.25)
    });
    case("hidden-5", || {
        // The first bad line wins, and it is returned as the AppError itself.
        expect_err(sum_pairs("1,1\n2\n3,x"), AppError::MissingComma, "missing comma")?;
        expect_err(
            sum_pairs("1,1\n2,x\n3"),
            parse_err(INVALID_FLOAT),
            "parse error: invalid float literal",
        )?;
        expect_err(
            sum_pairs("\n,2"),
            parse_err(EMPTY_INT),
            "parse error: cannot parse integer from empty string",
        )
    });
    case("hidden-6", || {
        // `a` is converted to f64 before it is added, so large values do not overflow.
        expect_sum(sum_pairs("2147483647,0.5\n2147483647,0.5"), 4294967295.0)
    });
}
