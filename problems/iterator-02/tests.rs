use std::panic::{self, AssertUnwindSafe};

use crate::judge::{case, check, check_eq};
use crate::user::StepRange;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

fn items(start: i64, end: i64, step: u64) -> Vec<i64> {
    StepRange::new(start, end, step).collect()
}

fn items_rev(start: i64, end: i64, step: u64) -> Vec<i64> {
    StepRange::new(start, end, step).rev().collect()
}

/// Checks that both `size_hint` and `len` say exactly `n` items are left.
fn check_len(r: &StepRange, n: usize) -> Result<(), String> {
    check_eq(r.size_hint(), (n, Some(n)))?;
    check_eq(r.len(), n)
}

/// Whether `f` panics.
fn panics(f: impl FnOnce()) -> bool {
    panic::catch_unwind(AssertUnwindSafe(f)).is_err()
}

pub fn run() {
    case("example-1", || {
        check_eq(items(-10, 10, 7), [-10, -3, 4])?;
        check_eq(items_rev(-10, 10, 7), [4, -3, -10])?;
        let mut r = StepRange::new(-10, 10, 7);
        check_eq(r.next(), Some(-10))?;
        check_eq(r.len(), 2)
    });
    case("example-2", || check_eq(items(MAX - 5, MAX, 4), [MAX - 5, MAX - 1]));
    case("example-3", || check_eq(items(MIN, MAX, u64::MAX), [MIN]));
    case("case-1", || {
        // Both ends in turn: they meet in the middle and never cross.
        let mut r = StepRange::new(0, 10, 3);
        check_len(&r, 4)?;
        check_eq(r.next(), Some(0))?;
        check_eq(r.next_back(), Some(9))?;
        check_len(&r, 2)?;
        check_eq(r.next(), Some(3))?;
        check_eq(r.next_back(), Some(6))?;
        check_len(&r, 0)?;
        check_eq([r.next(), r.next_back(), r.next()], [None; 3])?;
        // An odd count: the middle item is yielded once.
        let mut r = StepRange::new(0, 9, 3);
        check_eq(r.next_back(), Some(6))?;
        check_eq(r.next(), Some(0))?;
        check_len(&r, 1)?;
        check_eq(r.next_back(), Some(3))?;
        check_len(&r, 0)?;
        check_eq([r.next_back(), r.next(), r.next_back()], [None; 3])
    });
    case("case-2", || {
        // Empty ranges, also when `end - start` does not fit in an i64.
        for (start, end) in [(5, 5), (6, 5), (MAX, MAX), (MAX, MIN), (0, MIN)] {
            let mut r = StepRange::new(start, end, 1);
            check_len(&r, 0)?;
            check_eq([r.next(), r.next_back(), r.next()], [None; 3])?;
        }
        // A single item.
        check_eq(items(0, 1, u64::MAX), [0])?;
        check_eq(items_rev(MAX - 1, MAX, 2), [MAX - 1])?;
        let mut r = StepRange::new(7, 8, 3);
        check_len(&r, 1)?;
        check_eq(r.next_back(), Some(7))?;
        check_eq([r.next(), r.next_back()], [None, None])
    });
    case("case-3", || {
        // The back starts at the largest item below `end`, which is not always `end - step`.
        check_eq(items_rev(0, 10, 3), [9, 6, 3, 0])?;
        check_eq(items_rev(-7, 8, 5), [3, -2, -7])?;
        check_eq(items_rev(-5, 5, 1), [4, 3, 2, 1, 0, -1, -2, -3, -4, -5])?;
        check_eq(items(0, 11, 10), [0, 10])?;
        check_eq(items_rev(0, 10, 10), [0])?;
        check_eq(items_rev(MIN, MIN + 3, 2), [MIN + 2, MIN])?;
        let mut r = StepRange::new(MAX - 10, MAX, 3);
        check_eq(r.next_back(), Some(MAX - 1))?;
        check_eq(r.next(), Some(MAX - 10))?;
        check_eq(r.next(), Some(MAX - 7))?;
        check_eq(r.next(), Some(MAX - 4))?;
        check_eq([r.next(), r.next_back()], [None, None])
    });
    case("case-4", || {
        // A zero step panics, even for an empty range.
        check(panics(|| drop(StepRange::new(0, 10, 0))), "new(0, 10, 0) did not panic")?;
        check(panics(|| drop(StepRange::new(5, 5, 0))), "new(5, 5, 0) did not panic")
    });
    case("case-5", || {
        // Huge counts: up to 2^64 - 1 items.
        let mut r = StepRange::new(MIN, MAX, 1);
        check_len(&r, usize::MAX)?;
        check_eq(r.next_back(), Some(MAX - 1))?;
        check_eq(r.next(), Some(MIN))?;
        check_eq(r.next_back(), Some(MAX - 2))?;
        check_len(&r, usize::MAX - 3)?;
        let mut r = StepRange::new(MIN, MAX, 2);
        check_len(&r, 1 << 63)?;
        check_eq(r.next_back(), Some(MAX - 1))?;
        check_eq(r.next(), Some(MIN))?;
        let mut r = StepRange::new(MIN, MAX, 3);
        check_len(&r, usize::MAX / 3)?;
        check_eq(r.next_back(), Some(MAX - 3))?;
        check_eq(r.next(), Some(MIN))?;
        check_eq(r.next(), Some(MIN + 3))?;
        check_len(&r, usize::MAX / 3 - 3)
    });
    case("case-6", || {
        // Steps of 2^63 and more.
        check_eq(items(MIN, MAX, 1 << 63), [MIN, 0])?;
        check_eq(items_rev(MIN, MAX, 1 << 63), [0, MIN])?;
        check_eq(items(-1, MAX, 1 << 63), [-1])?;
        check_eq(items_rev(MIN + 1, 3, (1 << 63) + 1), [2, MIN + 1])?;
        check_eq(items(MIN, MAX, u64::MAX - 1), [MIN, MAX - 1])?;
        let mut r = StepRange::new(MIN, MAX, u64::MAX - 1);
        check_len(&r, 2)?;
        check_eq(r.next_back(), Some(MAX - 1))?;
        check_eq(r.next_back(), Some(MIN))?;
        check_eq([r.next(), r.next_back()], [None, None])
    });
    case("case-7", || {
        // Adapters that rely on next_back and on the exact length.
        let mut r = StepRange::new(0, 100, 7);
        check_len(&r, 15)?;
        r.next();
        r.next_back();
        check_eq(r.skip(2).len(), 11)?;
        check_eq(StepRange::new(0, 10, 3).skip(1).rev().collect::<Vec<_>>(), [9, 6, 3])?;
        let pairs: Vec<(usize, i64)> = StepRange::new(0, 10, 3).enumerate().rev().collect();
        check_eq(pairs, [(3, 9), (2, 6), (1, 3), (0, 0)])?;
        check_eq(StepRange::new(0, 100, 7).rposition(|x| x % 2 == 1), Some(13))?;
        check_eq(StepRange::new(0, 100, 7).rev().skip(3).take(4).collect::<Vec<_>>(), [77, 70, 63, 56])
    });
}
