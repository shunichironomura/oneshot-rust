use std::fmt;

use crate::judge::{case, check_eq};
use crate::user::pack_header;

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
    case("example-1", || check_hex(pack_header(0x12, 0x3456), 0x3456_0012));
    case("case-1", || {
        check_hex(pack_header(0, 0), 0)?;
        check_hex(pack_header(1, 0), 1)?;
        check_hex(pack_header(0, 1), 0x0001_0000)?;
        check_hex(pack_header(0x80, 0x8000), 0x8000_0080)?;
        check_hex(pack_header(0xFF, 0xFFFF), 0xFFFF_00FF)
    });
}
