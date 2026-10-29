use std::ops::Range;
use std::thread;

use crate::judge::{case, check};
use crate::user::{next_id, reserve};

/// One step of a worker: `None` calls `next_id()`, `Some(n)` calls `reserve(n)`.
type Step = Option<u64>;

const NEXT_ID: Step = None;

/// Runs the steps in order and returns the ids received by each step as a range, leaving out
/// empty ones.
fn work(steps: &[Step]) -> Result<Vec<Range<u64>>, String> {
    let mut got = Vec::with_capacity(steps.len());
    for &step in steps {
        let ids = match step {
            None => {
                let id = next_id();
                id..id + 1
            }
            Some(n) => {
                let r = reserve(n);
                if r.end.checked_sub(r.start) != Some(n) {
                    return Err("reserve(n) did not return n ids".to_string());
                }
                r
            }
        };
        if !ids.is_empty() {
            got.push(ids);
        }
    }
    Ok(got)
}

// The checks below index slices in plain loops: under Miri, that is much cheaper than sorting
// or long iterator chains.

/// Whether each range starts at or after the end of the one before it.
fn increasing(ranges: &[Range<u64>]) -> bool {
    let mut i = 1;
    while i < ranges.len() {
        if ranges[i].start < ranges[i - 1].end {
            return false;
        }
        i += 1;
    }
    true
}

/// Whether two lists of increasing ranges have no id in common.
fn disjoint(a: &[Range<u64>], b: &[Range<u64>]) -> bool {
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i].end <= b[j].start {
            i += 1;
        } else if b[j].end <= a[i].start {
            j += 1;
        } else {
            return false;
        }
    }
    true
}

/// Runs each list of steps on its own scoped thread and checks the ids they received.
fn check_threads(plans: &[&[Step]]) -> Result<(), String> {
    let got: Vec<Vec<Range<u64>>> = thread::scope(|s| {
        let handles: Vec<_> = plans.iter().map(|&steps| s.spawn(move || work(steps))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect::<Result<_, _>>()
    })?;
    for (t, ranges) in got.iter().enumerate() {
        check(increasing(ranges), "the ids of one thread do not increase")?;
        for other in &got[..t] {
            check(disjoint(ranges, other), "the same id was handed out twice")?;
        }
    }
    Ok(())
}

pub fn run() {
    case("example-1", || {
        let a = next_id();
        let r = reserve(3);
        let b = next_id();
        check(a < r.start, "a < r.start")?;
        check(r.end.checked_sub(r.start) == Some(3), "r has 3 ids")?;
        check(r.end <= b, "r.end <= b")
    });
    case("example-2", || check_threads(&[&[NEXT_ID; 3], &[NEXT_ID; 3], &[NEXT_ID; 3]]));
    case("case-1", || {
        // reserve(0) is empty, and the ids around it still increase.
        let a = next_id();
        let empty = reserve(0);
        let b = next_id();
        let one = reserve(1);
        let c = next_id();
        check(empty.is_empty(), "reserve(0) is not empty")?;
        check(a < b, "a < b")?;
        check(one.end.checked_sub(one.start) == Some(1), "reserve(1) does not have 1 id")?;
        check(b < one.start && one.end <= c, "b < one.start, one.end <= c")
    });
    case("case-2", || {
        // Consecutive reservations on one thread do not overlap.
        let r1 = reserve(5);
        let r2 = reserve(1000);
        let x = next_id();
        let r3 = reserve(2);
        check(r1.end.checked_sub(r1.start) == Some(5), "r1 has 5 ids")?;
        check(r2.end.checked_sub(r2.start) == Some(1000), "r2 has 1000 ids")?;
        check(r3.end.checked_sub(r3.start) == Some(2), "r3 has 2 ids")?;
        check(r1.end <= r2.start && r2.end <= x && x < r3.start, "ranges overlap")
    });
    case("case-3", || {
        // Three threads at once, each mixing next_id with reserve(n) for n in 0..=4 and 50.
        let steps: Vec<Step> = (0..40)
            .map(|i| match i % 10 {
                4 => Some(50),
                k if k % 2 == 0 => NEXT_ID,
                _ => Some(i % 5),
            })
            .collect();
        check_threads(&[&steps, &steps, &steps])
    });
}
