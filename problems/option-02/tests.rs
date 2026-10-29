use crate::judge::{case, check_eq};
use crate::user::within_limit;

pub fn run() {
    case("example-1", || {
        check_eq(within_limit(None, 99), true)?;
        check_eq(within_limit(Some(3), 2), true)?;
        check_eq(within_limit(Some(3), 3), false)
    });
    case("case-1", || {
        // No limit allows every count, even u32::MAX; Some(n) allows exactly n attempts.
        check_eq(within_limit(None, 0), true)?;
        check_eq(within_limit(None, u32::MAX), true)?;
        check_eq(within_limit(Some(0), 0), false)?;
        check_eq(within_limit(Some(1), 0), true)?;
        check_eq(within_limit(Some(3), 4), false)?;
        check_eq(within_limit(Some(u32::MAX), u32::MAX - 1), true)?;
        check_eq(within_limit(Some(u32::MAX), u32::MAX), false)
    });
}
