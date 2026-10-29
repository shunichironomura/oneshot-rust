use crate::judge::{case, check_eq};
use crate::user::powers_of;

pub fn run() {
    case("example-1", || {
        let tens = powers_of(10);
        check_eq(tens.len(), 20)?;
        check_eq(tens.first(), Some(&1))?;
        check_eq(tens.last(), Some(&10_000_000_000_000_000_000))?;
        check_eq(powers_of(2).last(), Some(&(1 << 63)))
    });
    case("case-1", || {
        let twos = powers_of(2);
        check_eq(twos.len(), 64)?;
        check_eq((twos[0], twos[1], twos[2], twos[63]), (1, 2, 4, 1 << 63))?;
        let threes = powers_of(3);
        check_eq(threes.len(), 41)?;
        check_eq((threes[0], threes[1], threes[2], threes[40]), (1, 3, 9, 12157665459056928801))?;
        check_eq(powers_of(10).last(), Some(&10_000_000_000_000_000_000))
    });
    case("case-2", || {
        // The largest power fits exactly or only just.
        check_eq(powers_of(u64::MAX), vec![1, u64::MAX])?;
        check_eq(powers_of(1 << 63), vec![1, 1 << 63])?;
        check_eq(powers_of(1 << 32), vec![1, 1 << 32])?;
        check_eq(powers_of(4294967295), vec![1, 4294967295, 18446744065119617025])?;
        // Powers of two whose next power would be exactly 2^64.
        check_eq(powers_of(65536), vec![1, 1 << 16, 1 << 32, 1 << 48])?;
        check_eq(powers_of(256).len(), 8)?;
        check_eq(powers_of(16).len(), 16)
    });
}
