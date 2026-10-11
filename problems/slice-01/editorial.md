Each function is one slice method away, and none of them needs a hand-written index loop.

- `chunk_by` (stable since Rust 1.77) calls the closure with every two neighbors `a` and `b`, and
  starts a new group wherever it returns `false`. With `a == b` the groups are the runs of equal
  values; with `a < b` they are the strictly increasing runs. Every group is a non-empty subslice of
  `v`, so `group[0]` is safe, and an empty `v` has no groups at all.
- `windows(3)` yields every three neighbors `[v[i], v[i + 1], v[i + 2]]`. Window `i` is centered on
  index `i + 1`, hence the `+ 1`. A slice shorter than three has no windows.
- `chunks_exact(2)` yields only full pairs, and `remainder()` returns what is left over (an empty
  slice or one element). `map` takes the iterator by value, so ask for `remainder()` **before**
  calling `map`. Calling it afterwards is a "borrow of moved value" error.

`rising_runs` returns the groups from `chunk_by` as they are, without copying. By lifetime elision,
`fn rising_runs(v: &[i32]) -> Vec<&[i32]>` means
`fn rising_runs<'a>(v: &'a [i32]) -> Vec<&'a [i32]>`: the slices in the result borrow from `v`,
which is exactly what `v.chunk_by(...)` yields.

The traps are in hand-written index loops:

- `for i in 1..v.len() - 1` looks harmless, but for an empty `v`, `v.len() - 1` is `0 - 1` on a
  `usize`. The judge builds with overflow checks, as debug builds do, so this panics with
  `attempt to subtract with overflow`. The same goes for `0..v.len() - 1` in `runs` or `pairs`.
- A loop that pushes the last run after the loop returns `[[]]` instead of `[]` for an empty `v`.
- Testing `b - a > 0` instead of `b > a` overflows when `i32::MIN` and `i32::MAX` are neighbors.
  Compare the values directly.

### Alternatives

Index loops work too, as long as the empty slice is handled. `saturating_sub` stops at `0` instead
of underflowing, and `runs` can extend the last group through `last_mut` without any index.

```rust
pub fn runs(v: &[i32]) -> Vec<(i32, usize)> {
    let mut result: Vec<(i32, usize)> = Vec::new();
    for &x in v {
        match result.last_mut() {
            Some((value, len)) if *value == x => *len += 1,
            _ => result.push((x, 1)),
        }
    }
    result
}

pub fn rising_runs(v: &[i32]) -> Vec<&[i32]> {
    let mut result = Vec::new();
    if v.is_empty() {
        return result;
    }
    let mut start = 0;
    for i in 1..v.len() {
        if v[i] <= v[i - 1] {
            result.push(&v[start..i]);
            start = i;
        }
    }
    result.push(&v[start..]);
    result
}

pub fn local_peaks(v: &[i32]) -> Vec<usize> {
    (1..v.len().saturating_sub(1))
        .filter(|&i| v[i - 1] < v[i] && v[i] > v[i + 1])
        .collect()
}
```

To read `remainder()` after the pairs, iterate with `by_ref()`, which only borrows the iterator.

```rust
pub fn pairs(v: &[i32]) -> (Vec<(i32, i32)>, Option<i32>) {
    let mut chunks = v.chunks_exact(2);
    let pairs = chunks.by_ref().map(|pair| (pair[0], pair[1])).collect();
    (pairs, chunks.remainder().first().copied())
}
```

`as_chunks` (stable since Rust 1.88) splits a slice into arrays of a fixed length and the rest, so an
array pattern can take each pair apart.

```rust
pub fn pairs(v: &[i32]) -> (Vec<(i32, i32)>, Option<i32>) {
    let (chunks, rest) = v.as_chunks::<2>();
    (chunks.iter().map(|&[a, b]| (a, b)).collect(), rest.first().copied())
}
```
