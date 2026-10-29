use crate::judge::{case, check_eq};
use crate::user::sum_checked;

pub fn run() {
    case("example-1", || {
        check_eq(sum_checked(&[100, 155]), Some(255))?;
        check_eq(sum_checked(&[200, 100]), None)
    });
    case("case-1", || {
        // Empty slices, sums that just fit, and long sums that would overflow a `u16` as well.
        check_eq(sum_checked(&[]), Some(0))?;
        check_eq(sum_checked(&[255]), Some(255))?;
        check_eq(sum_checked(&[100, 156]), None)?;
        check_eq(sum_checked(&[0, 0, 7]), Some(7))?;
        check_eq(sum_checked(&[255; 300]), None)
    });
}
