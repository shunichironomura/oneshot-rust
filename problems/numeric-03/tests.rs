use crate::judge::{case, check_eq};
use crate::user::wrap_index;

pub fn run() {
    case("example-1", || {
        check_eq(wrap_index(-1, 5), 4)?;
        check_eq(wrap_index(12, 5), 2)
    });
    case("case-1", || {
        // Negative positions, multiples of `len`, and the extreme `i64` values.
        check_eq(wrap_index(-10, 5), 0)?;
        check_eq(wrap_index(-11, 5), 4)?;
        check_eq(wrap_index(4, 5), 4)?;
        check_eq(wrap_index(5, 5), 0)?;
        check_eq(wrap_index(0, 1), 0)?;
        check_eq(wrap_index(-1, 1_000_000), 999_999)?;
        check_eq(wrap_index(i64::MIN, 7), 6)?;
        check_eq(wrap_index(i64::MAX, 7), 0)?;
        check_eq(wrap_index(i64::MAX, 1), 0)?;
        check_eq(wrap_index(i64::MIN, 1_000_000), 224_192)?;
        check_eq(wrap_index(i64::MAX, 1_000_000), 775_807)
    });
}
