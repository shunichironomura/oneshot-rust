use crate::judge::{case, check_eq};
use crate::user::{first_multiple_above, fizz_label};

pub fn run() {
    case("sample-1", || {
        check_eq(fizz_label(10), "Buzz")?;
        check_eq(fizz_label(7), "7")
    });
    case("sample-2", || {
        check_eq(first_multiple_above(7, 20), 21)?;
        check_eq(first_multiple_above(5, 10), 15)?;
        check_eq(first_multiple_above(3, 0), 3)
    });
    case("hidden-1", || {
        // 0 is a multiple of 15, and 15 is checked before 3 and 5.
        check_eq(fizz_label(0), "FizzBuzz")?;
        check_eq(fizz_label(3), "Fizz")?;
        check_eq(fizz_label(25), "Buzz")?;
        check_eq(fizz_label(45), "FizzBuzz")
    });
    case("hidden-2", || {
        check_eq(fizz_label(1), "1")?;
        check_eq(fizz_label(98), "98")?;
        check_eq(fizz_label(u32::MAX - 1), "4294967294")
    });
    case("hidden-3", || {
        // The answer is strictly greater than `limit`, even when `limit` is a multiple.
        check_eq(first_multiple_above(1, 0), 1)?;
        check_eq(first_multiple_above(10, 10), 20)?;
        check_eq(first_multiple_above(4, 3), 4)?;
        check_eq(first_multiple_above(1000, 999), 1000)
    });
    case("hidden-4", || {
        check_eq(first_multiple_above(6, 100), 102)?;
        check_eq(first_multiple_above(5_000_000_000, 12_000_000_000), 15_000_000_000)
    });
}
