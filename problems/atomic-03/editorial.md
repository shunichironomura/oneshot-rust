`record` takes `&self`, and several threads hold a `&Stats` at the same time, so the plain fields in
the editor cannot be updated: `self.count += 1` does not compile through a shared reference.
`&mut self` is no way out either: a `static`, like any value that several threads use at once, can
only be reached through shared references. The fields need interior mutability that is safe across
threads. `Cell<u64>` is not `Sync`, so `static GLOBAL: Stats` would no longer compile; without locks,
that leaves atomics. An `AtomicU64` can be changed through `&self`, it is `Sync`, and
`AtomicU64::new` is a `const fn`, so `new` still works in a `static`. (`AtomicU64::default()` would
not compile there: `Default::default` cannot be called in a `const fn`.)

Each update in `record` is a single read-modify-write operation: `fetch_add` for the count and the
sum, `fetch_max` and `fetch_min` for the extremes. Each one reads, combines and writes the value as
one indivisible step, so no change made by another thread can slip in between and get overwritten.

An atomic cannot hold `None`, so the extremes need starting values. `min` starts at `u64::MAX`,
which is larger than any sample, so the first `fetch_min` replaces it (a minimum that starts at `0`
stays `0`). `max` starts at `0`, but `0` cannot mean "no samples", since `0` is a valid sample: `max`
and `min` look at the count instead.

`Ordering::Relaxed` is enough everywhere. All the operations on one atomic happen in a single order
(its modification order), so read-modify-write operations never lose an update, whatever the
ordering. The results are read only after the recording threads have been joined, and joining a
thread makes everything it did visible to the joining thread. Stronger orderings matter when an
atomic tells another thread that *other* data is ready, like the flag of a spin lock.

Pitfalls:

- `if value > max.load(..) { max.store(value, ..) }` is not a data race (both operations are
  atomic), but it is a race condition: another thread can store a larger value between the load and
  the store, and the store then replaces it with a smaller one. `let c = count.load(..);
  count.store(c + 1, ..)` loses increments the same way. `SeqCst` does not help: an ordering cannot
  turn two operations into one.
- Treating "`max` is `0`" or "`sum` is `0`" as "no samples" fails when every sample is `0`.
- The four fields are updated one after another, so a reader running alongside `record` could see
  the new count with the old sum. The statement rules that out. If you need a consistent snapshot
  while recording goes on, keep all four numbers under one `Mutex` instead.

### Alternatives

Store the largest sample plus one, so that `0` means "no samples", and let `u64::MAX` mean "no
samples" for the minimum (no sample can be `u64::MAX`, since values are at most `1_000_000`). Then
`max` and `min` do not need the count: `checked_sub(1)` turns `0` into `None`.

```rust
pub struct Stats {
    count: AtomicU64,
    sum: AtomicU64,
    /// The largest sample plus one, or `0` while there are no samples.
    max_plus_one: AtomicU64,
    /// The smallest sample, or `u64::MAX` while there are no samples.
    min: AtomicU64,
}

impl Stats {
    pub const fn new() -> Stats {
        Stats {
            count: AtomicU64::new(0),
            sum: AtomicU64::new(0),
            max_plus_one: AtomicU64::new(0),
            min: AtomicU64::new(u64::MAX),
        }
    }

    pub fn record(&self, value: u64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum.fetch_add(value, Ordering::Relaxed);
        self.max_plus_one.fetch_max(value + 1, Ordering::Relaxed);
        self.min.fetch_min(value, Ordering::Relaxed);
    }

    // count and sum as in the solution

    pub fn max(&self) -> Option<u64> {
        self.max_plus_one.load(Ordering::Relaxed).checked_sub(1)
    }

    pub fn min(&self) -> Option<u64> {
        let min = self.min.load(Ordering::Relaxed);
        (min != u64::MAX).then_some(min)
    }
}
```

Without `fetch_max`, you would write a compare-and-swap loop: it retries until no other thread has
changed the value between the read and the write, or until the stored value is no longer smaller.
In `record`, call `raise_to(&self.max, value)`; `lower_to` for the minimum is the same with `<`.

```rust
fn raise_to(max: &AtomicU64, value: u64) {
    let mut current = max.load(Ordering::Relaxed);
    while value > current {
        match max.compare_exchange_weak(current, value, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            // Another thread changed the value in between (or the exchange failed spuriously):
            // try again with the value it holds now.
            Err(actual) => current = actual,
        }
    }
}
```

`update` (stable since Rust 1.95) runs such a loop around a closure, which works for any update
that does not have its own `fetch_*` method:

```rust
self.max.update(Ordering::Relaxed, Ordering::Relaxed, |max| max.max(value));
```
