use crate::judge::{case, check_eq};
use crate::user::local_peaks;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

fn check_peaks(v: &[i32], expected: &[usize]) -> Result<(), String> {
    check_eq(local_peaks(v), expected)
}

pub fn run() {
    case("example-1", || check_peaks(&[1, 3, 2, 5, 1], &[1, 3]));
    case("case-1", || {
        // Too short for a peak, without panicking.
        check_peaks(&[], &[])?;
        check_peaks(&[7], &[])?;
        check_peaks(&[1, 2], &[])
    });
    case("case-2", || {
        // Flat tops and valleys are not peaks; peaks can sit right next to either end.
        check_peaks(&[1, 4, 4, 1], &[])?;
        check_peaks(&[5, 5, 5, 5], &[])?;
        check_peaks(&[2, 1, 2], &[])?;
        check_peaks(&[1, 9, 1], &[1])?;
        check_peaks(&[9, 1, 2, 1], &[2])?;
        check_peaks(&[-5, -1, -3, -2, -4, -4, 0, -1], &[1, 3, 6])
    });
    case("case-3", || {
        // The extreme values of `i32`, next to each other.
        check_peaks(&[MIN, MAX, MIN], &[1])?;
        check_peaks(&[MAX, MIN, MAX, MAX, MIN], &[])
    });
}
