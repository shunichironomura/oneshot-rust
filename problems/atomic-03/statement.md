Several worker threads measure something (for example, how many microseconds each request took)
and record every measurement in a shared `Stats`. Implement `Stats` **without locks**.

```rust
pub struct Stats { /* fields of your choice */ }

impl Stats {
    pub const fn new() -> Stats;
    pub fn record(&self, value: u64);
    pub fn count(&self) -> u64;
    pub fn sum(&self) -> u64;
    pub fn max(&self) -> Option<u64>;
    pub fn min(&self) -> Option<u64>;
}

pub static GLOBAL: Stats = Stats::new();
```

- `new` returns a `Stats` with no samples.
- `record(value)` adds one sample, `value`. Every value is between `0` and `1_000_000` inclusive,
  and `0` is a sample like any other.
- `count` returns the number of samples, `sum` their total, `max` the largest and `min` the
  smallest. With no samples, `count` and `sum` are `0`, and `max` and `min` are `None`. You do not
  need to handle overflow.
- Every `Stats`, `GLOBAL` included, has its own samples.
- Several threads call `record` on the same `Stats` (through `&Stats`) at the same time. No sample
  may be lost, whatever the timing.
- `count`, `sum`, `max` and `min` are never called while a `record` call on the same `Stats` is
  running. They reflect every `record` call that finished before, including the calls made by
  threads that have been joined (for example, at the end of `std::thread::scope`).
- `new` must stay a `const fn`: the tests keep `Stats` values in `static`s, like `GLOBAL`.

Do not change the public signatures or `GLOBAL`. The fields of `Stats` (the ones in the editor are
only a starting point) and the body of `new` are up to you. Do not use locks such as `Mutex` or
`RwLock`.

### Examples

```rust
let stats = Stats::new();
// count() == 0, sum() == 0, max() == None, min() == None
stats.record(5);
stats.record(1);
stats.record(9);
// count() == 3, sum() == 15, max() == Some(9), min() == Some(1)
```

Four threads record the numbers from 1 to 100 into one `Stats`:

```rust
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
// count() == 100, sum() == 5050, max() == Some(100), min() == Some(1)
```
