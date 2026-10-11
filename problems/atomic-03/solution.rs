use std::sync::atomic::{AtomicU64, Ordering};

pub struct Stats {
    count: AtomicU64,
    /// std has no atomic `f64`, so the `f64` fields are stored as their bits (`f64::to_bits`).
    sum: AtomicU64,
    /// Starts at `-inf`, which is smaller than any sample, so the first sample replaces it.
    max: AtomicU64,
    /// Starts at `+inf`, which is larger than any sample.
    min: AtomicU64,
}

impl Stats {
    pub const fn new() -> Stats {
        Stats {
            count: AtomicU64::new(0),
            sum: AtomicU64::new(0.0_f64.to_bits()),
            max: AtomicU64::new(f64::NEG_INFINITY.to_bits()),
            min: AtomicU64::new(f64::INFINITY.to_bits()),
        }
    }

    pub fn record(&self, value: f64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        update_f64(&self.sum, |sum| sum + value);
        update_f64(&self.max, |max| max.max(value));
        update_f64(&self.min, |min| min.min(value));
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn sum(&self) -> f64 {
        load_f64(&self.sum)
    }

    pub fn max(&self) -> Option<f64> {
        if self.count() == 0 { None } else { Some(load_f64(&self.max)) }
    }

    pub fn min(&self) -> Option<f64> {
        if self.count() == 0 { None } else { Some(load_f64(&self.min)) }
    }
}

pub static GLOBAL: Stats = Stats::new();

fn load_f64(cell: &AtomicU64) -> f64 {
    f64::from_bits(cell.load(Ordering::Relaxed))
}

/// Replaces the `f64` stored in `cell` with `f(old)` as one atomic step. `update` retries `f` with
/// the new value whenever another thread changed `cell` between the load and the store.
fn update_f64(cell: &AtomicU64, f: impl Fn(f64) -> f64) {
    cell.update(Ordering::Relaxed, Ordering::Relaxed, |bits| f(f64::from_bits(bits)).to_bits());
}
