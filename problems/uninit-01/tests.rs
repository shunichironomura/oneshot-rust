use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

use crate::judge::{case, check, check_eq};
use crate::user::build;

/// How many times the `Tracked` value of each index has been dropped, 4 bits per index.
static DROPS: AtomicU64 = AtomicU64::new(0);

/// The value that `f` returns for an index in `trial`. It has no heap memory and counts how many
/// times it is dropped, so that a leak or a double drop shows up in `DROPS` even where Miri has
/// nothing to report.
struct Tracked(usize);

impl Drop for Tracked {
    fn drop(&mut self) {
        DROPS.fetch_add(1u64 << (4 * self.0), Relaxed);
    }
}

/// A trap that `trial` never hits.
const NONE: usize = usize::MAX;

/// Panics from inside `f`. `resume_unwind` unwinds exactly like `panic!` but skips the panic
/// hook, which keeps the tests fast under Miri.
fn fail() -> ! {
    panic::resume_unwind(Box::new("f panicked"))
}

/// Runs `f` and tells whether it panicked.
fn panics(f: impl FnOnce()) -> bool {
    panic::catch_unwind(AssertUnwindSafe(f)).is_err()
}

/// Checks that the `Tracked` value of each index `i < n` has been dropped `expected[i]` times.
fn check_drops(expected: &[u64]) -> Result<(), String> {
    let all = DROPS.load(Relaxed);
    let got: Vec<u64> = (0..expected.len()).map(|i| (all >> (4 * i)) & 15).collect();
    check_eq(&got[..], expected).map_err(|e| format!("drops of the value of each index: {e}"))
}

/// Calls `build(order, f)` with an `f` that returns `Tracked` values but panics when it is called
/// with `trap`, and checks every rule of the statement.
fn trial(order: &[usize], trap: usize) -> Result<(), String> {
    DROPS.store(0, Relaxed);
    let n = order.len();
    let mut calls = Vec::new();
    let mut built = None;
    let panicked = panics(|| {
        built = Some(build(order, |i| {
            calls.push(i);
            if i == trap {
                fail();
            }
            Tracked(i)
        }));
    });
    match order.iter().position(|&i| i == trap) {
        Some(p) => {
            check(panicked, "the panic of f did not reach the caller")?;
            check_eq(&calls[..], &order[..=p])?;
            // Each value produced before the panic has been dropped exactly once.
            let mut expected = vec![0; n];
            for &i in &order[..p] {
                expected[i] = 1;
            }
            check_drops(&expected)
        }
        None => {
            check(!panicked, "build panicked")?;
            check_eq(&calls[..], order)?;
            let v = built.unwrap();
            let indices: Vec<usize> = v.iter().map(|t| t.0).collect();
            check_eq(indices, (0..n).collect::<Vec<_>>())?;
            check_drops(&vec![0; n])?;
            // Dropping the vector drops each value once.
            drop(v);
            check_drops(&vec![1; n])
        }
    }
}

/// Like `trial` for a panic, with values in heap memory, whose leaks and double frees Miri reports.
fn boxed_panic(order: &[usize], trap: usize) -> Result<(), String> {
    let mut calls = Vec::new();
    let panicked = panics(|| {
        build(order, |i| {
            calls.push(i);
            if i == trap {
                fail();
            }
            Box::new(i)
        });
    });
    check(panicked, "the panic of f did not reach the caller")?;
    let p = order.iter().position(|&i| i == trap).unwrap();
    check_eq(&calls[..], &order[..=p])
}

pub fn run() {
    case("sample-1", || {
        let mut calls = Vec::new();
        let v = build(&[2, 0, 1], |i| {
            calls.push(i);
            i.to_string()
        });
        check_eq(calls, [2, 0, 1])?;
        check_eq(v, ["0", "1", "2"])
    });
    case("sample-2", || trial(&[3, 1, 0, 2], 0));
    case("hidden-1", || {
        // An empty order, a single index, and the order 0, 1, 2, ...
        let empty: Vec<String> = build(&[], |_| fail());
        check(empty.is_empty(), "the vector for an empty order is not empty")?;
        check_eq(build(&[0], |i| vec![i]), [[0]])?;
        check_eq(build(&[0, 1, 2, 3], |i| 10 * i as i32), [0, 10, 20, 30])?;
        trial(&[], NONE)
    });
    case("hidden-2", || {
        // Other types, and each value goes to the index `f` was called with.
        check_eq(build(&[4, 2, 0, 3, 1], |i| (i * i) as u64), [0, 1, 4, 9, 16])?;
        let mut k = 0;
        let v = build(&[2, 0, 1], |i| {
            k += 1;
            (i, k)
        });
        check_eq(v, [(0, 2), (1, 3), (2, 1)])?;
        let v = build(&[1, 2, 0], |i| Box::new(i as u8));
        check_eq(v, [Box::new(0), Box::new(1), Box::new(2)])?;
        check_eq(build(&[1, 0, 2], |_| ()), [(), (), ()])
    });
    case("hidden-3", || {
        // Without a panic, every value is dropped once, when the vector is dropped.
        trial(&[3, 0, 2, 1], NONE)?;
        trial(&[4, 3, 2, 1, 0], NONE)
    });
    case("hidden-4", || {
        // f panics on its first call: nothing has been produced yet.
        trial(&[1, 0, 2], 1)
    });
    case("hidden-5", || {
        // f panics in the middle of the work, also when the slots filled so far are scattered.
        trial(&[5, 0, 3, 1, 4, 2], 4)?;
        trial(&[0, 1, 2, 3], 2)?;
        boxed_panic(&[2, 0, 3, 1], 3)
    });
    case("hidden-6", || {
        // f panics on its last call: every value but one has been produced.
        trial(&[2, 0, 1, 3], 3)
    });
}
