use std::cell::Cell;
use std::thread;

use crate::judge::{case, check, check_eq};
use crate::user::{Memo, fib};

/// The recursive definition of `fib`, as a closure for `run_all`.
fn fib_step(m: &Memo, n: u64) -> u64 {
    if n < 2 { n } else { fib(m, n - 1) + fib(m, n - 2) }
}

pub fn run() {
    case("example-1", || {
        let m = Memo::new();
        check_eq(fib(&m, 10), 55)?;
        check_eq(m.misses(), 11)?;
        check_eq(fib(&m, 10), 55)?;
        check_eq(m.misses(), 11)
    });
    case("example-2", || {
        let m = Memo::new();
        m.schedule(12);
        m.schedule(5);
        check_eq(
            m.run_all(|_, k| k * 10),
            vec![(5, 50), (2, 20), (1, 10), (12, 120), (6, 60), (3, 30), (1, 10)],
        )?;
        check_eq(m.misses(), 6)
    });
    case("case-1", || {
        // f gets the key; a hit does not call f; 0 is a value like any other.
        let m = Memo::new();
        check_eq(m.get_or_compute(7, |_, k| k * 3), 21)?;
        check_eq(m.misses(), 1)?;
        let called = Cell::new(false);
        check_eq(m.get_or_compute(7, |_, _| { called.set(true); 0 }), 21)?;
        check(!called.get(), "f was called for a cached key")?;
        check_eq(m.get_or_compute(0, |_, _| 0), 0)?;
        check_eq(m.get_or_compute(0, |_, _| { called.set(true); 9 }), 0)?;
        check(!called.get(), "f was called for a cached key")?;
        check_eq(m.misses(), 2)
    });
    case("case-2", || {
        // f receives the memo itself and may use it while it runs.
        let m = Memo::new();
        let v = m.get_or_compute(1000, |m, k| m.get_or_compute(2000, |_, _| 40) + k / 500);
        check_eq(v, 42)?;
        check_eq(m.get_or_compute(2000, |_, _| 0), 40)?;
        check_eq(m.misses(), 2)
    });
    case("case-3", || {
        // An empty queue; run_all leaves the queue empty.
        let m = Memo::new();
        let calls = Cell::new(0);
        check_eq(m.run_all(|_, k| { calls.set(calls.get() + 1); k }), Vec::<(u64, u64)>::new())?;
        m.schedule(3);
        check_eq(m.run_all(|_, k| { calls.set(calls.get() + 1); k + 1 }), vec![(3, 4), (1, 2)])?;
        check_eq(m.run_all(|_, k| { calls.set(calls.get() + 1); k }), Vec::<(u64, u64)>::new())?;
        check_eq(calls.get(), 2)?;
        check_eq(m.misses(), 2)
    });
    case("case-4", || {
        // f schedules a key while run_all is running.
        let m = Memo::new();
        m.schedule(4);
        let got = m.run_all(|m, k| {
            if k == 4 {
                m.schedule(9);
            }
            k + 100
        });
        check_eq(got, vec![(4, 104), (2, 102), (1, 101), (9, 109), (4, 104), (2, 102), (1, 101)])?;
        check_eq(m.misses(), 4)
    });
    case("case-5", || {
        // A key scheduled by f is processed before the keys that were already waiting.
        let m = Memo::new();
        m.schedule(3);
        m.schedule(4);
        let got = m.run_all(|m, k| {
            if k == 4 {
                m.schedule(9);
            }
            k + 100
        });
        check_eq(
            got,
            vec![
                (4, 104),
                (2, 102),
                (1, 101),
                (9, 109),
                (4, 104),
                (2, 102),
                (1, 101),
                (3, 103),
                (1, 101),
            ],
        )?;
        check_eq(m.misses(), 5)
    });
    case("case-6", || {
        // f uses the cache while run_all is running; the cache is shared with fib.
        let m = Memo::new();
        m.schedule(5);
        check_eq(m.run_all(fib_step), vec![(5, 5), (2, 1), (1, 1)])?;
        check_eq(m.misses(), 6)?;
        check_eq(fib(&m, 4), 3)?;
        check_eq(m.misses(), 6)
    });
    case("case-7", || {
        // Several threads use one memo at the same time; one of them runs the queue.
        let m = Memo::new();
        let (queued, a, b) = thread::scope(|s| {
            let a = s.spawn(|| fib(&m, 5));
            let b = s.spawn(|| fib(&m, 5));
            m.schedule(4);
            let queued = m.run_all(fib_step);
            (queued, a.join().unwrap(), b.join().unwrap())
        });
        check_eq(queued, vec![(4, 3), (2, 1), (1, 1)])?;
        check_eq((a, b), (5, 5))?;
        let before = m.misses();
        check(before >= 6, "too few misses")?;
        check_eq(fib(&m, 5), 5)?;
        check_eq(m.misses(), before)
    });
}
