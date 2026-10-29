use crate::judge::{case, check_eq};
use crate::user::{count_long, first_negative, position_of_max};

pub fn run() {
    case("example-1", || {
        check_eq(first_negative(&[3, -1, -5]), Some(-1))?;
        check_eq(position_of_max(&[1, 9, 9]), Some(1))?;
        check_eq(count_long(&["a", "abc", "de"], 2), 2)
    });
    case("case-1", || {
        // 0 is not negative; no negative element gives None.
        check_eq(first_negative(&[0, 5, -2, -7]), Some(-2))?;
        check_eq(first_negative(&[0, 1, 2]), None)?;
        check_eq(first_negative(&[]), None)
    });
    case("case-2", || {
        // The first of several maximums wins.
        check_eq(position_of_max(&[]), None)?;
        check_eq(position_of_max(&[42]), Some(0))?;
        check_eq(position_of_max(&[5, 1, 5]), Some(0))?;
        check_eq(position_of_max(&[-3, -1, -2, -1]), Some(1))
    });
    case("case-3", || {
        // Lengths are in bytes: "日本" is 6 bytes long.
        check_eq(count_long(&["日本", "abc", "", "ab"], 3), 2)?;
        check_eq(count_long(&["", "x"], 0), 2)?;
        check_eq(count_long(&[], 1), 0)
    });
}
