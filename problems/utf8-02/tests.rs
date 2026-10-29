use crate::judge::{case, check_eq};
use crate::user::Utf8Stream;

/// Pushes the chunks one by one, checking what each `push` returns, then checks `finish`.
fn stream(steps: &[(&[u8], &str)], tail: &str) -> Result<(), String> {
    let mut s = Utf8Stream::new();
    for &(chunk, expected) in steps {
        check_eq(s.push(chunk), expected)?;
    }
    check_eq(s.finish(), tail)
}

pub fn run() {
    case("example-1", || {
        stream(&[(b"caf\xC3", "caf"), (b"\xA9 \xE2", "é "), (b"\x82\xAC!\xFF", "€!?")], "")?;
        stream(&[(&[b'x', 0xF0, 0x9F], "x")], "?")
    });
    case("case-1", || {
        // A byte that cannot be completed is replaced at once, not held back.
        stream(&[(b"a\xFF", "a?"), (b"\xC0", "?"), (b"b\xF5", "b?")], "")?;
        // Held-back bytes that turn out to be invalid.
        stream(
            &[
                (&[0xF0, 0x9F], ""),
                (b"A", "?A"),
                (&[0xED], ""),
                (&[0xA0], "??"),
                (&[0xF4], ""),
                (&[0x90, 0x80], "???"),
                (&[0xE2, 0x82], ""),
                (&[0xE2, 0x82], "?"),
                (&[0xAC], "€"),
            ],
            "",
        )
    });
    case("case-2", || {
        // Empty chunks, and bytes still held back at the end.
        stream(&[(b"", ""), (&[0xF0, 0x9F], ""), (b"", ""), (&[0x98, 0x80], "😀")], "")?;
        stream(&[(b"x\xE2\x82", "x")], "?")?;
        stream(&[(b"\xFF\xE2", "?")], "?")?;
        stream(&[], "")
    });
    case("case-3", || {
        // One byte at a time, the joined text is the same as decoding everything at once.
        let bytes: &[u8] = b"\xC3\xA9\xFF\xF0\x9F\x98\x80\xE2\x82?\xF0\x9F";
        let mut s = Utf8Stream::new();
        let mut text = String::new();
        for &b in bytes {
            text += &s.push(&[b]);
        }
        text += &s.finish();
        check_eq(text, "é?😀???")
    });
}
