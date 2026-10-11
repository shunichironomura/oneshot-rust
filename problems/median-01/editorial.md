`v` is a borrowed slice (`&[i32]`), so `v.sort()` does not compile: sorting needs `&mut`. Copy the
values into a `Vec` you own with `to_vec()` and sort that. `sort_unstable` is enough for integers,
since equal values cannot be told apart.

For an even length, do not add the two middle values as `i32`. `i32::MAX + i32::MAX` overflows,
which panics in debug builds, and integer division would drop the `.5` anyway. Convert each value
to `f64` first: an `f64` holds every `i32` exactly, and so does the sum of two of them (at most 2^32
in absolute value, far below 2^53, the limit for exact integers). Halving it is exact too.
`f64::from` is a lossless conversion; `as f64` gives the same result here.

For `mode`, `*counts.entry(x).or_insert(0) += 1` is the usual way to count. Ties are where picking
the winner goes wrong:

- A `HashMap` iterates in an arbitrary order, so "the first one found" means nothing.
- `max_by_key` returns the **last** of several equal maxima (and `min_by_key` the first).

So `max_by_key(|&(_, count)| count)` returns one of the tied values, and which one depends on the
hash order. Put the whole rule into the key instead. Tuples compare element by element, so
`(count, Reverse(value))` compares the counts first, and for equal counts the smaller value has the
larger `Reverse`. Writing `-value` instead of `Reverse(value)` looks equivalent, but negating
`i32::MIN` overflows.

### Alternatives

`mode` without a `HashMap`: in a sorted copy, equal values are next to each other, and `chunk_by`
yields each run of equal values. The runs come in ascending order and `min_by_key` returns the first
of several minima, so the key `Reverse(run.len())` picks the smallest of the most frequent values.

```rust
use std::cmp::Reverse;

pub fn mode(v: &[i32]) -> Option<i32> {
    let mut sorted = v.to_vec();
    sorted.sort_unstable();
    sorted
        .chunk_by(|a, b| a == b)
        .min_by_key(|run| Reverse(run.len()))
        .map(|run| run[0])
}
```

`median` can also add the middle values as `i64`, where the sum of two `i32` values always fits.
Converting that sum to `f64` is exact. Do not divide by 2 before converting: integer division turns
`-1 / 2` into `0`.

```rust
let sum = i64::from(sorted[mid - 1]) + i64::from(sorted[mid]);
Some(sum as f64 / 2.0)
```
