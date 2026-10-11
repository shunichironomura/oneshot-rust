use std::sync::Barrier;
use std::thread;

use crate::judge::{case, check_eq};
use crate::user::{GLOBAL, Stats};

/// Built with the `const fn new`, like `GLOBAL`.
static SHARED: Stats = Stats::new();

/// `(count, sum, max, min)`
type Summary = (u64, u64, Option<u64>, Option<u64>);

fn summary(stats: &Stats) -> Summary {
    (stats.count(), stats.sum(), stats.max(), stats.min())
}

const ROUNDS: usize = 10;
const BUCKETS_PER_ROUND: usize = 6;
const BUCKETS: usize = ROUNDS * BUCKETS_PER_ROUND;

/// In bucket `i`, thread `t` records `10 * i + ORDERS[i % 6][t]`, so the largest and the smallest
/// value of a bucket come from different threads from one bucket to the next.
const ORDERS: [[u64; 4]; 6] =
    [[0, 1, 2, 3], [3, 2, 1, 0], [1, 3, 0, 2], [2, 0, 3, 1], [3, 0, 2, 1], [1, 2, 3, 0]];

/// Four threads go through the buckets in order and wait for each other at the start of every
/// round, so that their calls on the same bucket overlap. Each thread records its value for a
/// bucket into the bucket and into `GLOBAL`. Then every bucket is checked.
fn record_in_buckets() -> Result<(), String> {
    let buckets: Vec<Stats> = (0..BUCKETS).map(|_| Stats::new()).collect();
    let together = Barrier::new(4);
    thread::scope(|s| {
        for t in 0..4 {
            let (buckets, together) = (&buckets, &together);
            s.spawn(move || {
                for (i, bucket) in buckets.iter().enumerate() {
                    if i % BUCKETS_PER_ROUND == 0 {
                        together.wait();
                    }
                    let value = 10 * i as u64 + ORDERS[i % 6][t];
                    GLOBAL.record(value);
                    bucket.record(value);
                }
            });
        }
    });
    for (i, bucket) in buckets.iter().enumerate() {
        let base = 10 * i as u64;
        check_eq(summary(bucket), (4, 4 * base + 6, Some(base + 3), Some(base)))?;
    }
    Ok(())
}

pub fn run() {
    case("sample-1", || {
        let stats = Stats::new();
        check_eq(summary(&stats), (0, 0, None, None))?;
        stats.record(5);
        stats.record(1);
        stats.record(9);
        check_eq(summary(&stats), (3, 15, Some(9), Some(1)))
    });
    case("sample-2", || {
        let stats = Stats::new();
        thread::scope(|s| {
            for t in 0..4 {
                let stats = &stats;
                s.spawn(move || {
                    for i in 1..=25 {
                        stats.record(t * 25 + i);
                    }
                });
            }
        });
        check_eq(summary(&stats), (100, 5050, Some(100), Some(1)))
    });
    case("hidden-1", || {
        // No samples: a new Stats, and GLOBAL, which the cases above did not touch.
        check_eq(summary(&Stats::new()), (0, 0, None, None))?;
        check_eq(summary(&GLOBAL), (0, 0, None, None))
    });
    case("hidden-2", || {
        // 0 is a sample like any other.
        let stats = Stats::new();
        stats.record(0);
        check_eq(summary(&stats), (1, 0, Some(0), Some(0)))?;
        stats.record(0);
        stats.record(0);
        check_eq(summary(&stats), (3, 0, Some(0), Some(0)))
    });
    case("hidden-3", || {
        // One sample is both the largest and the smallest; then the largest allowed value.
        SHARED.record(42);
        check_eq(summary(&SHARED), (1, 42, Some(42), Some(42)))?;
        SHARED.record(1_000_000);
        SHARED.record(7);
        check_eq(summary(&SHARED), (3, 1_000_049, Some(1_000_000), Some(7)))?;
        // Another Stats does not share the samples.
        let other = Stats::new();
        other.record(500);
        check_eq(summary(&other), (1, 500, Some(500), Some(500)))?;
        check_eq(summary(&SHARED), (3, 1_000_049, Some(1_000_000), Some(7)))
    });
    case("hidden-4", || {
        // Four threads recording at the same time, into many buckets and into GLOBAL.
        record_in_buckets()?;
        // GLOBAL got 10 * i + 0..=3 for every bucket i.
        let n = BUCKETS as u64;
        check_eq(summary(&GLOBAL), (4 * n, 20 * n * (n - 1) + 6 * n, Some(10 * n - 7), Some(0)))
    });
}
