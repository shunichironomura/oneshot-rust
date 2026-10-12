use crate::judge::{case, check_eq};
use crate::user::mode;

pub fn run() {
    case("sample-1", || {
        check_eq(mode(&[1, 3, 3, 1, 2]), Some(1))?;
        check_eq(mode(&[1, 2, 3, 3, 2, 3]), Some(3))
    });
    case("hidden-1", || {
        check_eq(mode(&[]), None)?;
        check_eq(mode(&[5]), Some(5))
    });
    case("hidden-2", || {
        // Ties: the smallest of the most frequent values wins.
        check_eq(mode(&[2, 2, 1, 1, 3, 3]), Some(1))?;
        check_eq(mode(&[50, 40, 30, 20, 10]), Some(10))?;
        check_eq(mode(&[3, 9, 2, 9, 3, 2, 7]), Some(2))?;
        check_eq(mode(&[-1, -2, -1, -2, 4, 4]), Some(-2))
    });
    case("hidden-3", || {
        // The most frequent value is neither the smallest nor the first one.
        check_eq(mode(&[5, 1, 5]), Some(5))?;
        check_eq(mode(&[4, 8, 8, 1, 6, 8, 6, 6, 6]), Some(6))
    });
    case("hidden-4", || {
        check_eq(mode(&[i32::MAX, i32::MIN, 0, i32::MIN, i32::MAX]), Some(i32::MIN))?;
        check_eq(mode(&[i32::MAX, 0, i32::MAX, 0, i32::MAX]), Some(i32::MAX))
    });
}
