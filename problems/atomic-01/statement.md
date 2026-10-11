This code compiles, and when you run it natively it seems to work. But it contains **undefined
behavior (UB)**, and Miri detects it.

`next_id` and `reserve` hand out ids (serial numbers) that must never repeat, for example to number
requests that several worker threads handle at the same time.

```rust
pub fn next_id() -> u64
pub fn reserve(n: u64) -> std::ops::Range<u64>
```

- `next_id` returns one new id.
- `reserve(n)` returns `n` new, consecutive ids as a range `start..start + n`. `n` may be `0`; the
  result is then an empty range.
- No id is ever handed out twice: the ids returned by `next_id` and the ids in the ranges returned by
  `reserve` are all different, across all threads and all calls.
- Within one thread, every id handed out is greater than all the ids that thread received before.
- Both functions may be called from several threads at the same time.
- The value of the first id is up to you. `n` is at most 1000, and you do not need to handle
  overflow.

Remove the UB so that it works correctly. (You may keep using `unsafe` if you like.)

### Examples

- On one thread, `let a = next_id(); let r = reserve(3); let b = next_id();` gives, for example,
  `a = 7`, `r = 8..11` and `b = 11`. If other threads take ids in between, the values can jump, but
  `a < r.start`, `r.end - r.start == 3` and `r.end <= b` always hold.
- Three threads, started with `std::thread::scope`, each call `next_id` three times. The nine ids
  are all different, and the three ids of each thread are increasing.
