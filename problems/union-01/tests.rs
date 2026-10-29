use std::fmt;

use crate::judge::{case, check_eq};
use crate::user::{byte_to_bool, float_bits, pack_header, widen};

/// A `u32` that failure messages show in hexadecimal.
#[derive(PartialEq)]
struct Hex(u32);

impl fmt::Debug for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010X}", self.0)
    }
}

fn check_hex(got: u32, expected: u32) -> Result<(), String> {
    check_eq(Hex(got), Hex(expected))
}

pub fn run() {
    case("example-1", || {
        check_hex(float_bits(1.0), 0x3F80_0000)?;
        check_hex(float_bits(-0.0), 0x8000_0000)?;
        check_eq(byte_to_bool(0), Some(false))?;
        check_eq(byte_to_bool(1), Some(true))
    });
    case("example-2", || check_hex(widen(0xAB), 0xAB));
    case("example-3", || check_hex(pack_header(0x12, 0x3456), 0x3456_0012));
    case("case-1", || {
        // The bits are kept exactly, also for special values.
        check_hex(float_bits(0.0), 0)?;
        check_hex(float_bits(-2.5), 0xC020_0000)?;
        check_hex(float_bits(f32::INFINITY), 0x7F80_0000)?;
        check_hex(float_bits(f32::NEG_INFINITY), 0xFF80_0000)?;
        check_hex(float_bits(f32::MAX), 0x7F7F_FFFF)?;
        check_hex(float_bits(f32::MIN_POSITIVE), 0x0080_0000)?;
        check_hex(float_bits(f32::from_bits(1)), 1)
    });
    case("case-2", || check_eq(byte_to_bool(2), None));
    case("case-3", || {
        // Every byte other than 0 and 1 gives None.
        for b in 0..=u8::MAX {
            let expected = match b {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            };
            check_eq(byte_to_bool(b), expected)?;
        }
        Ok(())
    });
    case("case-4", || {
        check_hex(widen(0), 0)?;
        check_hex(widen(1), 1)?;
        check_hex(widen(0x80), 0x80)?;
        check_hex(widen(u8::MAX), 0xFF)
    });
    case("case-5", || {
        check_hex(pack_header(0, 0), 0)?;
        check_hex(pack_header(1, 0), 1)?;
        check_hex(pack_header(0, 1), 0x0001_0000)?;
        check_hex(pack_header(0x80, 0x8000), 0x8000_0080)?;
        check_hex(pack_header(0xFF, 0xFFFF), 0xFFFF_00FF)
    });
}
