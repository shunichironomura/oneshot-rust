Several worker threads measure something (for example, how many milliseconds each request took,
or a temperature) and record every measurement in a shared `Stats`. Implement `Stats` **without
locks**.

```rust
pub struct Stats { /* fields of your choice */ }

impl Stats {
    pub const fn new() -> Stats;
    pub fn record(&self, value: f64);
    pub fn count(&self) -> u64;
    pub fn sum(&self) -> f64;
    pub fn max(&self) -> Option<f64>;
    pub fn min(&self) -> Option<f64>;
}

pub static GLOBAL: Stats = Stats::new();
```

- `new` returns a `Stats` with no samples.
- `record(value)` adds one sample, `value`. Every value is a finite number (never NaN or infinite)
  between `-1_000_000.0` and `1_000_000.0` inclusive. Negative values and `0.0` are samples like
  any other.
- `count` returns the number of samples, `sum` their total, `max` the largest and `min` the
  smallest. With no samples, `count` is `0`, `sum` is `0.0`, and `max` and `min` are `None`.
- The tests only use values such as `2.5` or `-0.75`, whose sums `f64` computes exactly, so the
  order in which the samples are added does not change `sum`.
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
// count() == 0, sum() == 0.0, max() == None, min() == None
stats.record(2.5);
stats.record(-1.0);
stats.record(4.0);
// count() == 3, sum() == 5.5, max() == Some(4.0), min() == Some(-1.0)
```

Four threads record the numbers `0.25`, `0.5`, `0.75`, ..., `10.0` into one `Stats`:

```rust
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
// count() == 40, sum() == 205.0, max() == Some(10.0), min() == Some(0.25)
```
