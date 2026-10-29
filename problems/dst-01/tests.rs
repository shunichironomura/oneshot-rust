use std::fmt::Debug;

use crate::judge::{case, check_eq};
use crate::user::byte_size;

// A type of the harness, with no `derive` and no `impl`.
struct Rgb(u8, u8, u8);

pub fn run() {
    case("example-1", || {
        check_eq(byte_size("héllo"), 6)?;
        check_eq(byte_size(&[1u16, 2, 3][..]), 6)?;
        check_eq(byte_size(&7u64), 8)
    });
    case("case-1", || {
        check_eq(byte_size(""), 0)?;
        check_eq(byte_size("日本"), 6)?;
        let words = String::from("Rust 🦀");
        check_eq(byte_size(&words[5..]), 4)?;
        let v = vec![1u32, 2, 3];
        check_eq(byte_size(&v[..]), 12)?;
        check_eq(byte_size(&v[..0]), 0)?;
        check_eq(byte_size(&[0u8; 5]), 5)
    });
    case("case-2", || {
        // For a trait object, the size of the value behind it.
        let d: &dyn Debug = &1u32;
        check_eq(byte_size(d), 4)?;
        let flags: &dyn Debug = &[true; 3];
        check_eq(byte_size(flags), 3)?;
        let pixels = vec![Rgb(0, 0, 0), Rgb(255, 128, 0)];
        check_eq(byte_size(&pixels[1]), 3)?;
        check_eq(byte_size(&pixels[..]), 6)
    });
}
