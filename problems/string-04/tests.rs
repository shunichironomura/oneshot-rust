use crate::judge::{case, check, check_eq};
use crate::user::last_chars;

/// Runs `last_chars` on a copy of `text` that sits between two other bytes of the same allocation,
/// and checks the value of the result and that it ends where the input does. A slice that does not
/// come from the input cannot point there.
fn last(text: &str, n: usize, expected: &str) -> Result<(), String> {
    let buf = ["[", text, "]"].concat();
    let s = &buf[1..buf.len() - 1];
    let got = last_chars(s, n);
    let end = |x: &str| x.as_bytes().as_ptr_range().end;
    let result = check_eq(got, expected)
        .and_then(|()| check(end(got) == end(s), "the result must end where s ends"));
    result.map_err(|e| format!("last_chars({text:?}, {n}): {e}"))
}

pub fn run() {
    case("example-1", || last("日本語です", 2, "です"));
    case("case-1", || {
        // n = 0: an empty suffix at the end.
        last("héllo", 0, "")?;
        last("日本", 0, "")
    });
    case("case-2", || {
        // n is at least the number of chars, and empty inputs.
        last("héllo", 5, "héllo")?;
        last("héllo", 9, "héllo")?;
        last("", 0, "")?;
        last("", 3, "")
    });
    case("case-3", || {
        // 4-byte chars, and a combining accent that is a char of its own.
        last("cafe\u{301}", 1, "\u{301}")?;
        last("cafe\u{301}", 2, "e\u{301}")?;
        last("🦀a🦀b", 3, "a🦀b")?;
        last("héllo", 4, "éllo")
    });
}
