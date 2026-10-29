use std::error::Error;
use std::num::ParseIntError;

use crate::judge::{case, check_eq};
use crate::user::total;

type Res = Result<i64, Box<dyn Error>>;

/// Expects `Ok(want)`.
fn expect_ok(got: Res, want: i64) -> Result<(), String> {
    match got {
        Ok(v) => check_eq(v, want),
        Err(e) => Err(format!("expected Ok({want}), got Err({e:?})")),
    }
}

/// Expects exactly the `ParseIntError` that parsing `line` as an `i64` produces.
fn expect_parse_err(got: Res, line: &str) -> Result<(), String> {
    let want = line.parse::<i64>().unwrap_err();
    match got {
        Ok(v) => Err(format!("expected Err({want:?}), got Ok({v})")),
        Err(e) => match e.downcast_ref::<ParseIntError>() {
            Some(got) => check_eq(got, &want),
            None => Err(format!(
                "expected Err({want:?}), got Err({e:?}), which is not a ParseIntError"
            )),
        },
    }
}

/// Expects an error whose `to_string()` is `msg`.
fn expect_msg(got: Res, msg: &str) -> Result<(), String> {
    match got {
        Ok(v) => Err(format!("expected an error displayed as {msg:?}, got Ok({v})")),
        Err(e) => {
            let shown = e.to_string();
            if shown == msg {
                Ok(())
            } else {
                Err(format!(
                    "expected an error displayed as {msg:?}, got Err({e:?}) displayed as {shown:?}"
                ))
            }
        }
    }
}

pub fn run() {
    case("example-1", || expect_ok(total("1\n# c\n\n 2 "), 3));
    case("example-2", || expect_parse_err(total("1\nx"), "x"));
    case("example-3", || expect_msg(total(""), "no numbers"));
    case("case-1", || {
        // Comments and blank lines only, also indented ones.
        expect_msg(total("# only"), "no numbers")?;
        expect_msg(total("  \n\t\n   # indented\n"), "no numbers")
    });
    case("case-2", || {
        // A sum of zero is not "no numbers".
        expect_ok(total("0"), 0)?;
        expect_ok(total("3\n-3\n"), 0)
    });
    case("case-3", || {
        expect_ok(total("\t+5 \n  # note\n-7\n\n\n"), -2)?;
        expect_ok(total("1\r\n2\r\n"), 3)
    });
    case("case-4", || {
        expect_msg(total("9223372036854775807\n1"), "overflow")?;
        expect_msg(total("-9223372036854775808\n-1"), "overflow")
    });
    case("case-5", || {
        // The running sum must stay in range, even if later numbers would bring it back.
        expect_msg(total("9223372036854775807\n1\n-1"), "overflow")?;
        expect_ok(total("9223372036854775807\n-1\n1"), i64::MAX)?;
        expect_ok(total("# min\n-9223372036854775808"), i64::MIN)
    });
    case("case-6", || {
        // The first problem wins.
        expect_parse_err(total("x\n9223372036854775807\n1"), "x")?;
        expect_msg(total("9223372036854775807\n1\nx"), "overflow")
    });
    case("case-7", || {
        expect_parse_err(total("1 2"), "1 2")?;
        expect_parse_err(total("4\n5 # five"), "5 # five")?;
        expect_parse_err(total("99999999999999999999"), "99999999999999999999")?;
        expect_parse_err(total("-"), "-")
    });
}
