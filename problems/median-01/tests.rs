use crate::judge::{case, check_eq};
use crate::user::median;

pub fn run() {
    case("example-1", || {
        check_eq(median(&[3, 1, 2]), Some(2.0))?;
        check_eq(median(&[4, 1, 3, 2]), Some(2.5))
    });
    case("case-1", || check_eq(median(&[]), None));
    case("case-2", || {
        check_eq(median(&[7]), Some(7.0))?;
        check_eq(median(&[-7]), Some(-7.0))
    });
    case("case-3", || {
        // Unsorted input with duplicates, and means of negative values.
        check_eq(median(&[5, 1, 5, 3, 9]), Some(5.0))?;
        check_eq(median(&[10, -4, 7, -1]), Some(3.0))?;
        check_eq(median(&[-3, -1]), Some(-2.0))?;
        check_eq(median(&[-1, -4]), Some(-2.5))?;
        check_eq(median(&[8, 2, 6, 9, 1, 4]), Some(5.0))
    });
    case("case-4", || {
        // Extreme values: adding them as i32 overflows.
        check_eq(median(&[i32::MAX, i32::MAX]), Some(2147483647.0))?;
        check_eq(median(&[i32::MIN, i32::MIN]), Some(-2147483648.0))?;
        check_eq(median(&[i32::MIN, i32::MAX]), Some(-0.5))?;
        check_eq(median(&[i32::MAX, 0, i32::MAX - 1, i32::MAX]), Some(2147483646.5))?;
        check_eq(median(&[i32::MIN, 0, i32::MIN]), Some(-2147483648.0))
    });
}
