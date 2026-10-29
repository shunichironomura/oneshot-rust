use crate::judge::{case, check, check_eq};
use crate::user::{local_peaks, pairs, rising_runs, runs};

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

fn check_runs(v: &[i32], expected: &[(i32, usize)]) -> Result<(), String> {
    check_eq(runs(v), expected)
}

fn check_rising(v: &[i32], expected: &[&[i32]]) -> Result<(), String> {
    check_eq(rising_runs(v), expected)
}

fn check_peaks(v: &[i32], expected: &[usize]) -> Result<(), String> {
    check_eq(local_peaks(v), expected)
}

fn check_pairs(v: &[i32], expected: &[(i32, i32)], rest: Option<i32>) -> Result<(), String> {
    let (got, got_rest) = pairs(v);
    check_eq((got.as_slice(), got_rest), (expected, rest))
}

pub fn run() {
    case("example-1", || {
        check_runs(&[1, 1, 2, 3, 3, 3], &[(1, 2), (2, 1), (3, 3)])?;
        check_rising(&[1, 2, 2, 3, 1], &[&[1, 2], &[2, 3], &[1]])
    });
    case("example-2", || {
        check_peaks(&[1, 3, 2, 5, 1], &[1, 3])?;
        check_pairs(&[1, 2, 3, 4, 5], &[(1, 2), (3, 4)], Some(5))
    });
    case("case-1", || {
        // An empty slice gives empty results, without panicking.
        check_runs(&[], &[])?;
        check_rising(&[], &[])?;
        check_peaks(&[], &[])?;
        check_pairs(&[], &[], None)
    });
    case("case-2", || {
        // One and two elements: no peaks yet, and the pairs either fill up or leave one over.
        check_runs(&[7], &[(7, 1)])?;
        check_runs(&[4, 4], &[(4, 2)])?;
        check_runs(&[4, -4], &[(4, 1), (-4, 1)])?;
        check_rising(&[7], &[&[7]])?;
        check_rising(&[1, 2], &[&[1, 2]])?;
        check_rising(&[2, 1], &[&[2], &[1]])?;
        check_peaks(&[7], &[])?;
        check_peaks(&[1, 2], &[])?;
        check_pairs(&[7], &[], Some(7))?;
        check_pairs(&[1, 2], &[(1, 2)], None)
    });
    case("case-3", || {
        // Only neighbors are grouped: equal values further apart start new groups.
        check_runs(&[5, 5, 5, 5, 5], &[(5, 5)])?;
        check_runs(&[1, 2, 1, 1, 2], &[(1, 1), (2, 1), (1, 2), (2, 1)])?;
        check_runs(&[0, -3, -3, 0, 0, 0, 9], &[(0, 1), (-3, 2), (0, 3), (9, 1)])
    });
    case("case-4", || {
        // Equal neighbors split a rising run, and so does any decrease.
        check_rising(&[5, 5, 5], &[&[5], &[5], &[5]])?;
        check_rising(&[5, 4, 3, 2], &[&[5], &[4], &[3], &[2]])?;
        check_rising(&[-3, -1, 0, 8, 9], &[&[-3, -1, 0, 8, 9]])?;
        check_rising(&[3, 1, 2, 2, 5, 9, 0], &[&[3], &[1, 2], &[2, 5, 9], &[0]])?;
        // The runs are subslices of `v`.
        let v = [4, 6, 1, 3];
        let got = rising_runs(&v);
        check(
            got.len() == 2 && got[1].as_ptr() == v[2..].as_ptr(),
            "the runs should be subslices of v",
        )
    });
    case("case-5", || {
        // Flat tops and valleys are not peaks; peaks can sit right next to either end.
        check_peaks(&[1, 4, 4, 1], &[])?;
        check_peaks(&[5, 5, 5, 5], &[])?;
        check_peaks(&[2, 1, 2], &[])?;
        check_peaks(&[1, 9, 1], &[1])?;
        check_peaks(&[9, 1, 2, 1], &[2])?;
        check_peaks(&[-5, -1, -3, -2, -4, -4, 0, -1], &[1, 3, 6])
    });
    case("case-6", || {
        // An even length leaves nothing over; a leftover 0 is still `Some(0)`.
        check_pairs(&[1, 2, 3, 4, 5, 6], &[(1, 2), (3, 4), (5, 6)], None)?;
        check_pairs(&[0, 0, 0], &[(0, 0)], Some(0))?;
        check_pairs(&[9, 8, 7, 6, 5, 4, 3], &[(9, 8), (7, 6), (5, 4)], Some(3))
    });
    case("case-7", || {
        // The extreme values of `i32`, next to each other.
        check_runs(&[MIN, MIN, MAX, MAX, MAX, 0], &[(MIN, 2), (MAX, 3), (0, 1)])?;
        check_rising(&[MIN, MAX, MIN, 0, MAX], &[&[MIN, MAX], &[MIN, 0, MAX]])?;
        check_rising(&[MAX, MIN], &[&[MAX], &[MIN]])?;
        check_peaks(&[MIN, MAX, MIN], &[1])?;
        check_peaks(&[MAX, MIN, MAX, MAX, MIN], &[])?;
        check_pairs(&[MAX, MIN, MIN], &[(MAX, MIN)], Some(MIN))
    });
}
