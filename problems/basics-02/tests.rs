use crate::judge::{case, check_eq};
use crate::user::fizz_label;

pub fn run() {
    case("example-1", || {
        check_eq(fizz_label(10), "Buzz")?;
        check_eq(fizz_label(7), "7")
    });
    case("case-1", || {
        // 0 is a multiple of 15, and 15 is checked before 3 and 5.
        check_eq(fizz_label(0), "FizzBuzz")?;
        check_eq(fizz_label(3), "Fizz")?;
        check_eq(fizz_label(25), "Buzz")?;
        check_eq(fizz_label(45), "FizzBuzz")
    });
    case("case-2", || {
        check_eq(fizz_label(1), "1")?;
        check_eq(fizz_label(98), "98")?;
        check_eq(fizz_label(u32::MAX - 1), "4294967294")
    });
}
