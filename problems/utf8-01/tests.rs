use crate::judge::{case, check, check_eq};
use crate::user::{DecodeError, decode, decode_lossy};

use DecodeError::{Incomplete, Invalid};

fn lossy(bytes: &[u8], text: &str, count: usize) -> Result<(), String> {
    let (got, n) = decode_lossy(bytes);
    check_eq((got.as_str(), n), (text, count))
}

pub fn run() {
    case("example-1", || {
        check_eq(decode("hé".as_bytes()), Ok("hé"))?;
        check_eq(decode(b"hi\xFFthere"), Err(Invalid { at: 2, len: 1 }))?;
        check_eq(decode(&[0xF0, 0x9F, 0x98, b'A']), Err(Invalid { at: 0, len: 3 }))?;
        check_eq(decode(&[b'a', 0xE2, 0x82]), Err(Incomplete { at: 1 }))
    });
    case("example-2", || {
        lossy(b"ab\xFFcd", "ab?cd", 1)?;
        lossy(&[0xC0, 0xAF], "??", 2)?;
        lossy(&[0xF0, 0x9F, 0x98, b'A'], "?A", 1)?;
        lossy(b"\xEF\xBF\xBD?", "\u{FFFD}?", 0)?;
        lossy(&[b'a', 0xE2, 0x82], "a?", 1)
    });
    case("case-1", || {
        // Valid input, borrowed rather than copied.
        let bytes = "日本語 😀 ok".as_bytes();
        let text = decode(bytes);
        check_eq(text, Ok("日本語 😀 ok"))?;
        check(text.is_ok_and(|t| t.as_ptr() == bytes.as_ptr()), "the text should borrow the bytes")?;
        check_eq(decode(&[]), Ok(""))?;
        // Bytes that can never start a character, even at the end.
        check_eq(decode(&[0xC0, 0xAF]), Err(Invalid { at: 0, len: 1 }))?;
        check_eq(decode(b"ok\xC0"), Err(Invalid { at: 2, len: 1 }))?;
        check_eq(decode(&[0x80]), Err(Invalid { at: 0, len: 1 }))?;
        check_eq(decode(&[b'z', 0xF5]), Err(Invalid { at: 1, len: 1 }))
    });
    case("case-2", || {
        // Sequences that start like a character but are not one.
        check_eq(decode(b"ok\xED\xA0\x80"), Err(Invalid { at: 2, len: 1 }))?;
        check_eq(decode(&[0xE0, 0x80, 0x80]), Err(Invalid { at: 0, len: 1 }))?;
        check_eq(decode(&[0xF4, 0x90, 0x80, 0x80]), Err(Invalid { at: 0, len: 1 }))?;
        check_eq(decode(&[b'x', 0xE2, 0x82, b'y']), Err(Invalid { at: 1, len: 2 }))?;
        check_eq(decode(&[0xED, 0xA0]), Err(Invalid { at: 0, len: 1 }))?;
        // Only the first problem is reported.
        check_eq(decode(&[0xFF, 0xE2, 0x82]), Err(Invalid { at: 0, len: 1 }))?;
        check_eq(decode(b"\xC3\xA9\xEF\xBF\xBD\xF0\x9F\x98\xFF"), Err(Invalid { at: 5, len: 3 }))
    });
    case("case-3", || {
        // Incomplete characters at the end.
        check_eq(decode(&[0xE2, 0x82]), Err(Incomplete { at: 0 }))?;
        check_eq(decode(&[0xF0, 0x9F, 0x98]), Err(Incomplete { at: 0 }))?;
        check_eq(decode(b"\xC3\xBC\xED"), Err(Incomplete { at: 2 }))?;
        check_eq(decode(&[b'a', b'b', 0xF4]), Err(Incomplete { at: 2 }))?;
        check_eq(decode(&[0xE0, 0xA0]), Err(Incomplete { at: 0 }))
    });
    case("case-4", || {
        // Correctly encoded U+FFFD and '?' are kept and not counted.
        lossy("\u{FFFD}x\u{FFFD}".as_bytes(), "\u{FFFD}x\u{FFFD}", 0)?;
        lossy(b"\xEF\xBF\xBD\xFF?\xEF\xBF\xBD\xC3", "\u{FFFD}??\u{FFFD}?", 2)?;
        lossy(b"what?", "what?", 0)
    });
    case("case-5", || {
        lossy(&[], "", 0)?;
        lossy("héllo wörld 😀".as_bytes(), "héllo wörld 😀", 0)?;
        lossy(&[0xED, 0xA0, 0x80], "???", 3)?;
        lossy(&[0xF4, 0x90, 0x80, 0x80], "????", 4)?;
        lossy(&[0xE0, 0x80, 0x80], "???", 3)?;
        lossy(&[0x80, 0xBF], "??", 2)?;
        lossy(&[0xE2, 0x82, b'x', 0xE2, 0x82], "?x?", 2)?;
        lossy(&[0xF0, 0x9F, 0x98, 0xF0, 0x9F, 0x98, 0x80], "?😀", 1)
    });
}
