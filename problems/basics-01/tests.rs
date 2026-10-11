use crate::judge::{case, check_eq};
use crate::user::{digital_root, first_multiple_above, fizz_label};

pub fn run() {
    case("sample-1", || check_eq(digital_root(9875), 2));
    case("sample-2", || {
        check_eq(fizz_label(10), "Buzz")?;
        check_eq(fizz_label(7), "7")
    });
    case("sample-3", || {
        check_eq(first_multiple_above(7, 20), 21)?;
        check_eq(first_multiple_above(5, 10), 15)?;
        check_eq(first_multiple_above(3, 0), 3)
    });
    case("hidden-1", || {
        // Values below 10 are already their own digital root, including 0.
        check_eq(digital_root(0), 0)?;
        check_eq(digital_root(7), 7)?;
        check_eq(digital_root(9), 9)
    });
    case("hidden-2", || {
        // Several rounds may be needed: 199 -> 19 -> 10 -> 1.
        check_eq(digital_root(10), 1)?;
        check_eq(digital_root(99), 9)?;
        check_eq(digital_root(199), 1)?;
        check_eq(digital_root(u64::MAX), 6)
    });
    case("hidden-3", || {
        // 0 is a multiple of 15, and 15 is checked before 3 and 5.
        check_eq(fizz_label(0), "FizzBuzz")?;
        check_eq(fizz_label(3), "Fizz")?;
        check_eq(fizz_label(25), "Buzz")?;
        check_eq(fizz_label(45), "FizzBuzz")
    });
    case("hidden-4", || {
        check_eq(fizz_label(1), "1")?;
        check_eq(fizz_label(98), "98")?;
        check_eq(fizz_label(u32::MAX - 1), "4294967294")
    });
    case("hidden-5", || {
        // The answer is strictly greater than `limit`, even when `limit` is a multiple.
        check_eq(first_multiple_above(1, 0), 1)?;
        check_eq(first_multiple_above(10, 10), 20)?;
        check_eq(first_multiple_above(4, 3), 4)?;
        check_eq(first_multiple_above(1000, 999), 1000)
    });
    case("hidden-6", || {
        check_eq(first_multiple_above(6, 100), 102)?;
        check_eq(first_multiple_above(5_000_000_000, 12_000_000_000), 15_000_000_000)
    });
}
