use crate::judge::{case, check_eq};
use crate::user::{parse_hex_color, to_hex};

fn parses(s: &str, expected: [u8; 3]) -> Result<(), String> {
    check_eq(parse_hex_color(s), Some(expected)).map_err(|e| format!("parse_hex_color({s:?}): {e}"))
}

/// Every input must give `None` (a panic fails the case too).
fn rejects(inputs: &[&str]) -> Result<(), String> {
    for &s in inputs {
        if let Some(rgb) = parse_hex_color(s) {
            return Err(format!("parse_hex_color({s:?}): expected None, got Some({rgb:?})"));
        }
    }
    Ok(())
}

fn hex(rgb: [u8; 3], expected: &str) -> Result<(), String> {
    check_eq(to_hex(rgb), expected).map_err(|e| format!("to_hex({rgb:?}): {e}"))
}

pub fn run() {
    case("example-1", || {
        parses("#0A0b0C", [10, 11, 12])?;
        parses("#f80", [255, 136, 0])
    });
    case("example-2", || rejects(&["ff8800", "#ff80"]));
    case("example-3", || {
        hex([255, 8, 0], "#ff0800")?;
        hex([255, 136, 0], "#f80")
    });
    case("case-1", || {
        // Both forms, the whole range, channels in order, mixed case.
        parses("#000000", [0, 0, 0])?;
        parses("#FFFFFF", [255, 255, 255])?;
        parses("#123456", [0x12, 0x34, 0x56])?;
        parses("#7fFf80", [127, 255, 128])?;
        parses("#09afAF", [9, 175, 175])?;
        parses("#000", [0, 0, 0])?;
        parses("#FfF", [255, 255, 255])?;
        parses("#9aB", [0x99, 0xaa, 0xbb])
    });
    case("case-2", || {
        // No '#' at the start, a second '#', and every other number of digits.
        rejects(&["", "#", "#ffff", "#fffff", "#fffffff", "#ffffffff", "ffff", "##fff"])
    });
    case("case-3", || {
        // Whitespace is not trimmed, and other ASCII characters are not hex digits.
        rejects(&[" #fff", "#fff ", "#ff fff", "#ggg", "#12345g"])
    });
    case("case-4", || {
        // Signs are not hex digits, even where integer parsing accepts them.
        rejects(&["#+f+f+f", "#12+345", "#+fff00", "#-1-1-1", "#+ff"])
    });
    case("case-5", || {
        // Non-ASCII characters, some with the byte length of a valid code.
        rejects(&["#1é234", "#€", "#ab€d", "éfff", "#٣٣٣"])
    });
    case("case-6", || {
        // The 6-digit form keeps every leading zero and uses lowercase digits.
        hex([0, 0, 1], "#000001")?;
        hex([10, 171, 12], "#0aab0c")?;
        hex([0x10, 0x20, 0x30], "#102030")?;
        hex([0x88, 0x88, 0x80], "#888880")
    });
    case("case-7", || {
        // The 3-digit form when every channel has two equal digits.
        hex([0, 0, 0], "#000")?;
        hex([0x11, 0xaa, 0xcc], "#1ac")?;
        check_eq(parse_hex_color("#FfFfFf").map(to_hex), Some("#fff".to_string()))
    });
}
