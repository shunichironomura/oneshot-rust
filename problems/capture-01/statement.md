The code below does not compile. Fix it.

`fill_squares` sets every element of `out` to the square of its index. The work is split across
`workers` threads: worker `w` writes the elements at indices `w`, `w + workers`, `w + 2 * workers`,
and so on.

```rust
pub fn fill_squares(out: &mut [u64], workers: usize)
```

- When `fill_squares` returns, `out[i]` is `i * i` for every index `i` of `out`, whatever it held
  before. Indices count from the start of `out`, even when `out` is part of a larger slice.
- `workers` is at least 1 and may be larger than `out.len()`, so a worker may have nothing to write.
  `out` may be empty.
- Every thread has finished before `fill_squares` returns.

Do not change the signature. The work must stay split across separate threads (at most `workers` of
them), but you may divide the elements among them differently. You may keep the `unsafe` code.

### Example

```rust
let mut v = [7; 5];
fill_squares(&mut v, 2);
// v is now [0, 1, 4, 9, 16]: worker 0 wrote indices 0, 2 and 4, and worker 1 wrote 1 and 3.
```
