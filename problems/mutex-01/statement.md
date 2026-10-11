This code compiles, but it never finishes: Miri stops it with the error "the evaluated program
deadlocked".

`Memo` caches computed values (memoization) for recursive and concurrent computations, and keeps a
queue of keys waiting to be processed. The whole implementation is already in the editor.

```rust
pub struct Memo {
    cache: Mutex<BTreeMap<u64, u64>>,
    queue: Mutex<Vec<u64>>,
    misses: AtomicUsize,
}

impl Memo {
    pub fn new() -> Self;
    pub fn get_or_compute(&self, key: u64, f: impl FnOnce(&Memo, u64) -> u64) -> u64;
    pub fn misses(&self) -> usize;
    pub fn schedule(&self, key: u64);
    pub fn run_all(&self, f: impl Fn(&Memo, u64) -> u64) -> Vec<(u64, u64)>;
}

pub fn fib(m: &Memo, n: u64) -> u64 {
    m.get_or_compute(n, |m, n| if n < 2 { n } else { fib(m, n - 1) + fib(m, n - 2) })
}
```

- `Memo::new()` creates a memo with no cached values, an empty queue and a miss count of 0.
- `get_or_compute(key, f)` returns the value cached for `key` if there is one, without calling `f`.
  Otherwise (a miss) it calls `f(self, key)` once, caches the value that `f` returns for `key`, adds
  1 to the miss count and returns the value. A memo has a single cache, shared by all calls,
  including the ones made through `fib` and `run_all`.
- While `f` runs, it may use the memo it receives: it may call `get_or_compute` for other keys (as
  `fib` does), `schedule` and `misses`. It never asks for the key it is computing, never calls
  `run_all` and never panics.
- `misses()` returns the miss count.
- `schedule(key)` adds `key` to the queue. A key may be in the queue several times.
- `run_all(f)` processes the queue until it is empty, and returns one pair `(key, value)` per
  processed key, in processing order. Each step does the following, in this order:
  1. Take the most recently scheduled key `k` out of the queue (last in, first out).
  2. Get its value `v` with `get_or_compute`. On a miss, the value is computed by calling `f` with
     the memo and `k`.
  3. Append `(k, v)` to the result.
  4. If `k > 1`, schedule `k / 2` (rounded down).

  Keys scheduled while `run_all` is running, including those scheduled by `f`, are processed by the
  same call. On an empty queue, `run_all` returns an empty `Vec`.
- The harness shares one memo between several threads (with `std::thread::scope`). They may call
  `get_or_compute` (directly, through `fib` or through `run_all`) at the same time, but only one
  thread at a time uses the queue. If several threads miss the same key at the same time, each of
  them may compute it and count a miss: the harness checks the values that such concurrent calls
  return, but not how many misses they count.

Fix the code so that it works as described. Do not change the fields of `Memo`, the public
signatures or `fib`. You may add private helpers.

### Examples

```rust
let m = Memo::new();
fib(&m, 10)   // 55
m.misses()    // 11: fib(0) to fib(10) are each computed once
fib(&m, 10)   // 55, from the cache
m.misses()    // still 11

let m = Memo::new();
m.schedule(12);
m.schedule(5);
m.run_all(|_, k| k * 10)
// [(5, 50), (2, 20), (1, 10), (12, 120), (6, 60), (3, 30), (1, 10)]
m.misses()    // 6 (the second 1 is found in the cache)
```
