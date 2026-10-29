use std::borrow::Cow;

use crate::judge::{case, check, check_eq};
use crate::user::normalize_ws;

fn borrowed(got: Cow<'_, str>, expected: &str) -> Result<(), String> {
    match got {
        Cow::Borrowed(s) => check_eq(s, expected),
        Cow::Owned(s) => Err(format!("expected Borrowed({expected:?}), got Owned({s:?})")),
    }
}

fn owned(got: Cow<'_, str>, expected: &str) -> Result<(), String> {
    match got {
        Cow::Owned(s) => check_eq(s.as_str(), expected),
        Cow::Borrowed(s) => Err(format!("expected Owned({expected:?}), got Borrowed({s:?})")),
    }
}

pub fn run() {
    case("example-1", || {
        borrowed(normalize_ws("a b"), "a b")?;
        borrowed(normalize_ws("  hi\n"), "hi")?;
        owned(normalize_ws("a  b\tc "), "a b c")
    });
    case("case-1", || {
        // Every kind of ASCII whitespace, runs of plain spaces; trimming alone borrows a slice
        // of the input.
        borrowed(normalize_ws(""), "")?;
        borrowed(normalize_ws(" \t\r\n\x0C"), "")?;
        owned(normalize_ws("a\tb"), "a b")?;
        owned(normalize_ws(" a  b   c "), "a b c")?;
        owned(normalize_ws("\tx \r\n y\x0Cz  "), "x y z")?;
        let s = String::from("\n  one two three \t");
        match normalize_ws(&s) {
            Cow::Borrowed(t) => {
                check_eq(t, "one two three")?;
                check(t.as_ptr() == s[3..].as_ptr(), "the result should be a slice of the input")
            }
            Cow::Owned(t) => Err(format!("expected Borrowed, got Owned({t:?})")),
        }
    });
}
