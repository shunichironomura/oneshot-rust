use crate::judge::{case, check, check_eq};
use crate::user::{ellipsize, truncate_bytes};

/// Runs `truncate_bytes` on a copy of `text` that sits between two other bytes of the same
/// allocation, and checks the value of the result and that it starts where the input does. A slice
/// that does not come from the input cannot point there.
fn trunc(text: &str, max: usize, expected: &str) -> Result<(), String> {
    let buf = ["[", text, "]"].concat();
    let s = &buf[1..buf.len() - 1];
    let got = truncate_bytes(s, max);
    let result = check_eq(got, expected)
        .and_then(|()| check(got.as_ptr() == s.as_ptr(), "the result must start where s starts"));
    result.map_err(|e| format!("truncate_bytes({text:?}, {max}): {e}"))
}

fn ell(text: &str, max: usize, expected: &str) -> Result<(), String> {
    check_eq(ellipsize(text, max), expected).map_err(|e| format!("ellipsize({text:?}, {max}): {e}"))
}

pub fn run() {
    case("example-1", || {
        trunc("héllo", 2, "h")?;
        trunc("héllo", 3, "hé")
    });
    case("example-2", || {
        ell("こんにちは", 10, "こん…")?;
        ell("hello!", 5, "he…")?;
        ell("hello", 5, "hello")
    });
    case("case-1", || {
        // A char that does not fit is left out entirely; the result may be empty.
        trunc("🦀🦀", 5, "🦀")?;
        trunc("🦀🦀", 7, "🦀")?;
        trunc("🦀🦀", 8, "🦀🦀")?;
        trunc("🦀🦀", 3, "")?;
        trunc("héllo", 0, "")?;
        trunc("日本語", 6, "日本")?;
        trunc("日本語", 8, "日本")
    });
    case("case-2", || {
        // max at or beyond the length.
        trunc("abc", 3, "abc")?;
        trunc("abc", 10, "abc")?;
        trunc("", 0, "")?;
        trunc("", 5, "")?;
        ell("abc", 3, "abc")?;
        ell("", 3, "")?;
        ell("🦀🦀", 8, "🦀🦀")?;
        ell("héllo", 100, "héllo")
    });
    case("case-3", || {
        // Only the ellipsis fits, or the cut falls inside a char.
        ell("abcdef", 3, "…")?;
        ell("abcd", 3, "…")?;
        ell("🦀🦀", 7, "🦀…")?;
        ell("🦀🦀", 6, "…")?;
        ell("héllo", 5, "h…")?;
        ell("日本語です", 14, "日本語…")
    });
}
