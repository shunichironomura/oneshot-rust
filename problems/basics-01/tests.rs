use crate::judge::{case, check_eq};
use crate::user::digital_root;

pub fn run() {
    case("sample-1", || check_eq(digital_root(9875), 2));
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
}
