`record` takes `&self`, and several threads hold a `&Stats` at the same time, so the plain fields in
the editor cannot be updated: `self.count += 1` does not compile through a shared reference. `Cell`
is not `Sync`, so `static GLOBAL: Stats` would stop compiling, and locks are not allowed. That
leaves atomics, and the count is easy: an `AtomicU64` and `fetch_add`.

**The trap is the three `f64` fields: std has no atomic floating-point type.** There is no
`AtomicF64`, but an `f64` is 64 bits, and `f64::to_bits` and `f64::from_bits` turn it into a `u64`
and back without changing a single bit. So each `f64` lives in an `AtomicU64` as its bit pattern.
Both conversions are `const fn`, so `new` stays a `const fn`.

The atomic only knows integers, though, so its arithmetic is wrong for the numbers that the bits
encode:

- `sum.fetch_add(value.to_bits(), ..)` adds two bit patterns as integers. The bits of `2.5` plus
  the bits of `4.0` are not the bits of `6.5`, not even of a nearby number.
- For non-negative numbers, the bit patterns happen to be ordered like the numbers, so
  `max.fetch_max(value.to_bits(), ..)` passes simple tests. But the sign is the highest bit: every
  negative number has larger bits than every positive one, and among negative numbers, the one
  further from zero has the larger bits. With `-1.25` and `-8.0`, that "maximum" is `-8.0`.

So the new value must be computed as an `f64` and written back in one atomic step: a
compare-and-swap loop. `AtomicU64::update` (stable since Rust 1.95) runs the loop for you. It loads
the bits, calls the closure, and stores the result only if the atomic still holds the bits it
loaded; if another thread changed them in between, it calls the closure again with the new value.
The model solution wraps it in `update_f64`, which converts in both directions, and uses it for the
sum (`sum + value`), the maximum (`max.max(value)`) and the minimum (`min.min(value)`).

Pitfalls:

- `let s = f64::from_bits(sum.load(..)); sum.store((s + value).to_bits(), ..)` is not a data race
  (both operations are atomic), but it is a race condition: another thread can record between the
  load and the store, and its sample is lost. The same goes for a maximum that is loaded, compared
  and stored. `SeqCst` does not help: an ordering cannot turn two operations into one.
- An atomic cannot hold `None`, so the extremes need starting values. The maximum starts at
  `-inf` and the minimum at `+inf`, so the first sample replaces them; a maximum that starts at
  `0.0` stays `0.0` when every sample is negative. `max` and `min` return `None` while the count is
  `0`. A sum (or a maximum) of `0.0` does not mean "no samples": `0.0` is a valid sample.
- `f64` addition is not associative in general: with arbitrary values, the sum would depend on the
  order in which the threads happen to add. That is why the statement only uses values whose sums
  are exact.

`Ordering::Relaxed` is enough everywhere. All the changes to one atomic happen in a single order
(its modification order), and a successful compare-and-swap writes right after the value it read
in that order, so no update is lost. The results are read only after the recording threads have
been joined, and joining a thread makes everything it did visible to the joining thread.

The four fields are updated one after another, so a reader running alongside `record` could see
the new count with the old sum. The statement rules that out. If you need a consistent snapshot
while recording goes on, keep all four numbers under one `Mutex` instead.

### Alternatives

The loop that `update` runs, written by hand. `compare_exchange_weak` may fail even when the value
has not changed (a spurious failure), which only costs one more round of the loop.

```rust
fn update_f64(cell: &AtomicU64, f: impl Fn(f64) -> f64) {
    let mut current = cell.load(Ordering::Relaxed);
    loop {
        let new = f(f64::from_bits(current)).to_bits();
        match cell.compare_exchange_weak(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            // Another thread changed the value in between (or the exchange failed spuriously):
            // try again with the value it holds now.
            Err(actual) => current = actual,
        }
    }
}
```

`fetch_max` and `fetch_min` do work on a key whose integer order is the order of the numbers. Flip
the sign bit of a non-negative number, so that it lands above every negative one, and flip all the
bits of a negative number, which also reverses their order. (`f64::total_cmp` uses a similar
trick.) The key `0` is below the key of every number and `u64::MAX` above, so they can mean
"no samples". The sum still needs the compare-and-swap loop, here with `update`.

```rust
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Stats {
    count: AtomicU64,
    sum: AtomicU64,
    /// The key of the largest sample, or `0` while there are no samples.
    max_key: AtomicU64,
    /// The key of the smallest sample, or `u64::MAX` while there are no samples.
    min_key: AtomicU64,
}

const SIGN: u64 = 1 << 63;

/// Maps a number (not NaN) to a `u64` that is ordered like the numbers.
fn key(x: f64) -> u64 {
    let bits = x.to_bits();
    if bits & SIGN == 0 { bits | SIGN } else { !bits }
}

/// The inverse of `key`.
fn from_key(key: u64) -> f64 {
    f64::from_bits(if key & SIGN != 0 { key & !SIGN } else { !key })
}

impl Stats {
    pub const fn new() -> Stats {
        Stats {
            count: AtomicU64::new(0),
            sum: AtomicU64::new(0),
            max_key: AtomicU64::new(0),
            min_key: AtomicU64::new(u64::MAX),
        }
    }

    pub fn record(&self, value: f64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum.update(Ordering::Relaxed, Ordering::Relaxed, |bits| {
            (f64::from_bits(bits) + value).to_bits()
        });
        self.max_key.fetch_max(key(value), Ordering::Relaxed);
        self.min_key.fetch_min(key(value), Ordering::Relaxed);
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn sum(&self) -> f64 {
        f64::from_bits(self.sum.load(Ordering::Relaxed))
    }

    pub fn max(&self) -> Option<f64> {
        let max_key = self.max_key.load(Ordering::Relaxed);
        (max_key != 0).then(|| from_key(max_key))
    }

    pub fn min(&self) -> Option<f64> {
        let min_key = self.min_key.load(Ordering::Relaxed);
        (min_key != u64::MAX).then(|| from_key(min_key))
    }
}

pub static GLOBAL: Stats = Stats::new();
```
