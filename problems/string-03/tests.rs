use crate::judge::{case, check, check_eq};
use crate::user::{ellipsize, first_chars, last_chars, truncate_bytes};

/// Runs `f` on a copy of `text` that sits between two other bytes of the same allocation, and
/// checks the value of the result and that it starts (`at_start`) or ends (`at_end`) where the
/// input does. A slice that does not come from the input cannot point to either place.
fn check_slice(
    name: &str,
    text: &str,
    arg: usize,
    f: fn(&str, usize) -> &str,
    expected: &str,
    at_start: bool,
) -> Result<(), String> {
    let buf = ["[", text, "]"].concat();
    let s = &buf[1..buf.len() - 1];
    let got = f(s, arg);
    let result = check_eq(got, expected).and_then(|()| {
        if at_start {
            check(got.as_ptr() == s.as_ptr(), "the result must start where s starts")
        } else {
            let end = |x: &str| x.as_bytes().as_ptr_range().end;
            check(end(got) == end(s), "the result must end where s ends")
        }
    });
    result.map_err(|e| format!("{name}({text:?}, {arg}): {e}"))
}

fn first(text: &str, n: usize, expected: &str) -> Result<(), String> {
    check_slice("first_chars", text, n, first_chars, expected, true)
}

fn last(text: &str, n: usize, expected: &str) -> Result<(), String> {
    check_slice("last_chars", text, n, last_chars, expected, false)
}

fn trunc(text: &str, max: usize, expected: &str) -> Result<(), String> {
    check_slice("truncate_bytes", text, max, truncate_bytes, expected, true)
}

fn ell(text: &str, max: usize, expected: &str) -> Result<(), String> {
    check_eq(ellipsize(text, max), expected).map_err(|e| format!("ellipsize({text:?}, {max}): {e}"))
}

pub fn run() {
    case("sample-1", || {
        first("héllo", 2, "hé")?;
        last("日本語です", 2, "です")
    });
    case("sample-2", || {
        trunc("héllo", 2, "h")?;
        trunc("héllo", 3, "hé")
    });
    case("sample-3", || {
        ell("こんにちは", 10, "こん…")?;
        ell("hello!", 5, "he…")?;
        ell("hello", 5, "hello")
    });
    case("hidden-1", || {
        // n = 0: an empty prefix at the start, an empty suffix at the end.
        first("héllo", 0, "")?;
        last("héllo", 0, "")?;
        last("日本", 0, "")
    });
    case("hidden-2", || {
        // n is at least the number of chars, and empty inputs.
        first("héllo", 5, "héllo")?;
        first("héllo", 10, "héllo")?;
        last("héllo", 5, "héllo")?;
        last("héllo", 9, "héllo")?;
        first("", 0, "")?;
        first("", 3, "")?;
        last("", 0, "")?;
        last("", 3, "")
    });
    case("hidden-3", || {
        // 4-byte chars, and a combining accent that is a char of its own.
        first("e\u{301}cole", 1, "e")?;
        last("cafe\u{301}", 1, "\u{301}")?;
        last("cafe\u{301}", 2, "e\u{301}")?;
        first("🦀a🦀b", 3, "🦀a🦀")?;
        last("🦀a🦀b", 3, "a🦀b")?;
        last("héllo", 4, "éllo")
    });
    case("hidden-4", || {
        // A char that does not fit is left out entirely; the result may be empty.
        trunc("🦀🦀", 5, "🦀")?;
        trunc("🦀🦀", 7, "🦀")?;
        trunc("🦀🦀", 8, "🦀🦀")?;
        trunc("🦀🦀", 3, "")?;
        trunc("héllo", 0, "")?;
        trunc("日本語", 6, "日本")?;
        trunc("日本語", 8, "日本")
    });
    case("hidden-5", || {
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
    case("hidden-6", || {
        // Only the ellipsis fits, or the cut falls inside a char.
        ell("abcdef", 3, "…")?;
        ell("abcd", 3, "…")?;
        ell("🦀🦀", 7, "🦀…")?;
        ell("🦀🦀", 6, "…")?;
        ell("héllo", 5, "h…")?;
        ell("日本語です", 14, "日本語…")
    });
}
