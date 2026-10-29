use crate::judge::{case, check, check_eq};
use crate::user::rising_runs;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

fn check_rising(v: &[i32], expected: &[&[i32]]) -> Result<(), String> {
    check_eq(rising_runs(v), expected)
}

pub fn run() {
    case("example-1", || check_rising(&[1, 2, 2, 3, 1], &[&[1, 2], &[2, 3], &[1]]));
    case("case-1", || {
        // An empty slice gives no runs, without panicking.
        check_rising(&[], &[])
    });
    case("case-2", || {
        // One and two elements.
        check_rising(&[7], &[&[7]])?;
        check_rising(&[1, 2], &[&[1, 2]])?;
        check_rising(&[2, 1], &[&[2], &[1]])
    });
    case("case-3", || {
        // Equal neighbors split a rising run, and so does any decrease.
        check_rising(&[5, 5, 5], &[&[5], &[5], &[5]])?;
        check_rising(&[5, 4, 3, 2], &[&[5], &[4], &[3], &[2]])?;
        check_rising(&[-3, -1, 0, 8, 9], &[&[-3, -1, 0, 8, 9]])?;
        check_rising(&[3, 1, 2, 2, 5, 9, 0], &[&[3], &[1, 2], &[2, 5, 9], &[0]])
    });
    case("case-4", || {
        // The runs are subslices of `v`.
        let v = [4, 6, 1, 3];
        let got = rising_runs(&v);
        check(
            got.len() == 2 && got[1].as_ptr() == v[2..].as_ptr(),
            "the runs should be subslices of v",
        )
    });
    case("case-5", || {
        // The extreme values of `i32`, next to each other.
        check_rising(&[MIN, MAX, MIN, 0, MAX], &[&[MIN, MAX], &[MIN, 0, MAX]])?;
        check_rising(&[MAX, MIN], &[&[MAX], &[MIN]])
    });
}
