use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};

// An atomic can be changed through a shared reference, so a plain `static` is enough.
static NEXT: AtomicU64 = AtomicU64::new(0);

pub fn next_id() -> u64 {
    // `fetch_add` returns the value before the addition. `Relaxed` is enough: the calls still
    // take turns on `NEXT`, so no two of them see the same value.
    NEXT.fetch_add(1, Ordering::Relaxed)
}

pub fn reserve(n: u64) -> Range<u64> {
    // Claim all `n` ids with a single atomic operation.
    let start = NEXT.fetch_add(n, Ordering::Relaxed);
    start..start + n
}
