use std::sync::Barrier;
use std::thread;

use crate::judge::{case, check_eq};
use crate::user::{GLOBAL, Stats};

/// Built with the `const fn new`, like `GLOBAL`.
static SHARED: Stats = Stats::new();

/// `(count, sum, max, min)`
type Summary = (u64, f64, Option<f64>, Option<f64>);

fn summary(stats: &Stats) -> Summary {
    (stats.count(), stats.sum(), stats.max(), stats.min())
}

const ROUNDS: usize = 3;
const BUCKETS_PER_ROUND: usize = 6;
const K: usize = 3;
const BUCKETS: usize = ROUNDS * BUCKETS_PER_ROUND;

/// In bucket `i`, thread `t` records `base(i) + ORDERS[i % 6][t] / 4`, so the largest and the
/// smallest value of a bucket come from different threads from one bucket to the next.
const ORDERS: [[u32; 4]; 6] =
    [[0, 1, 2, 3], [3, 2, 1, 0], [1, 3, 0, 2], [2, 0, 3, 1], [3, 0, 2, 1], [1, 2, 3, 0]];

/// The smallest value of bucket `i`: `-25.0`, `-22.5`, `-20.0`, ... All the values recorded here
/// are multiples of `0.25`, so every sum of them is exact.
fn base(i: usize) -> f64 {
    (i as f64 - 10.0) * 2.5
}

/// Four threads go through the buckets in order and wait for each other at the start of every
/// round, so that their calls on the same bucket overlap. Bucket 0 is `GLOBAL`. Then every bucket
/// is checked.
fn record_in_buckets() -> Result<(), String> {
    let locals: Vec<Stats> = (1..BUCKETS).map(|_| Stats::new()).collect();
    let buckets: Vec<&Stats> = std::iter::once(&GLOBAL).chain(&locals).collect();
    let together = Barrier::new(4);
    thread::scope(|s| {
        for t in 0..4 {
            let (buckets, together) = (&buckets, &together);
            s.spawn(move || {
                for (i, bucket) in buckets.iter().enumerate() {
                    if i % BUCKETS_PER_ROUND == 0 {
                        together.wait();
                    }
                    for k in 0..K {
                        bucket.record(base(i) + (4 * k) as f64 / 4.0 + f64::from(ORDERS[i % 6][t]) / 4.0);
                    }
                }
            });
        }
    });
    for (i, bucket) in buckets.iter().enumerate() {
        let b = base(i);
        let n = 4 * K;
        check_eq(summary(bucket), (n as u64, n as f64 * b + ((n - 1) * n / 2) as f64 / 4.0, Some(b + (n - 1) as f64 / 4.0), Some(b)))?;
    }
    Ok(())
}

pub fn run() {
    case("sample-1", || {
        let stats = Stats::new();
        check_eq(summary(&stats), (0, 0.0, None, None))?;
        stats.record(2.5);
        stats.record(-1.0);
        stats.record(4.0);
        check_eq(summary(&stats), (3, 5.5, Some(4.0), Some(-1.0)))
    });
    case("sample-2", || {
        let stats = Stats::new();
        thread::scope(|s| {
            for t in 0..4 {
                let stats = &stats;
                s.spawn(move || {
                    for i in 1..=10 {
                        stats.record((t * 10 + i) as f64 / 4.0);
                    }
                });
            }
        });
        check_eq(summary(&stats), (40, 205.0, Some(10.0), Some(0.25)))
    });
    case("hidden-1", || {
        // No samples: a new Stats, and GLOBAL, which the cases above did not touch.
        check_eq(summary(&Stats::new()), (0, 0.0, None, None))?;
        check_eq(summary(&GLOBAL), (0, 0.0, None, None))
    });
    case("hidden-2", || {
        // 0.0 is a sample like any other.
        let stats = Stats::new();
        stats.record(0.0);
        check_eq(summary(&stats), (1, 0.0, Some(0.0), Some(0.0)))?;
        stats.record(0.0);
        stats.record(0.0);
        check_eq(summary(&stats), (3, 0.0, Some(0.0), Some(0.0)))
    });
    case("hidden-3", || {
        // Only negative samples: the largest one is below 0.
        let stats = Stats::new();
        stats.record(-3.5);
        stats.record(-1.25);
        stats.record(-8.0);
        check_eq(summary(&stats), (3, -12.75, Some(-1.25), Some(-8.0)))
    });
    case("hidden-4", || {
        // One sample is both the largest and the smallest; then the extreme values and a mix of
        // signs.
        SHARED.record(42.5);
        check_eq(summary(&SHARED), (1, 42.5, Some(42.5), Some(42.5)))?;
        SHARED.record(1_000_000.0);
        SHARED.record(-1_000_000.0);
        SHARED.record(-0.75);
        check_eq(summary(&SHARED), (4, 41.75, Some(1_000_000.0), Some(-1_000_000.0)))?;
        // Another Stats does not share the samples.
        let other = Stats::new();
        other.record(500.25);
        check_eq(summary(&other), (1, 500.25, Some(500.25), Some(500.25)))?;
        check_eq(summary(&SHARED), (4, 41.75, Some(1_000_000.0), Some(-1_000_000.0)))
    });
    case("hidden-5", || {
        // Four threads recording at the same time, into many buckets, GLOBAL included.
        record_in_buckets()
    });
}
