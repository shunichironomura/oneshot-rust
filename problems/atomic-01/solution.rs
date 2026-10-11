use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

pub fn next_id() -> u64 {
    NEXT.fetch_add(1, Ordering::Relaxed)
}

pub fn reserve(n: u64) -> Range<u64> {
    let start = NEXT.fetch_add(n, Ordering::Relaxed);
    start..start + n
}
