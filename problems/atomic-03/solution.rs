use std::sync::atomic::{AtomicU64, Ordering};

pub struct Stats {
    count: AtomicU64,
    sum: AtomicU64,
    /// Starts at `0`. That is also a valid sample, so `max()` asks the count whether there is one.
    max: AtomicU64,
    /// Starts at `u64::MAX`, which is larger than any sample, so the first `fetch_min` replaces it.
    min: AtomicU64,
}

impl Stats {
    pub const fn new() -> Stats {
        Stats {
            count: AtomicU64::new(0),
            sum: AtomicU64::new(0),
            max: AtomicU64::new(0),
            min: AtomicU64::new(u64::MAX),
        }
    }

    pub fn record(&self, value: u64) {
        // Each line is one indivisible read-modify-write, so no update from another thread is lost.
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum.fetch_add(value, Ordering::Relaxed);
        self.max.fetch_max(value, Ordering::Relaxed);
        self.min.fetch_min(value, Ordering::Relaxed);
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn sum(&self) -> u64 {
        self.sum.load(Ordering::Relaxed)
    }

    pub fn max(&self) -> Option<u64> {
        if self.count() == 0 {
            None
        } else {
            Some(self.max.load(Ordering::Relaxed))
        }
    }

    pub fn min(&self) -> Option<u64> {
        if self.count() == 0 {
            None
        } else {
            Some(self.min.load(Ordering::Relaxed))
        }
    }
}

pub static GLOBAL: Stats = Stats::new();
