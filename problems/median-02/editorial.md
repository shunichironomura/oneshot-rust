`*counts.entry(x).or_insert(0) += 1` is the usual way to count with a `HashMap`. Ties are where
picking the winner goes wrong:

- A `HashMap` iterates in an arbitrary order, so "the first one found" means nothing.
- `max_by_key` returns the **last** of several equal maxima (and `min_by_key` the first).

So `max_by_key(|&(_, count)| count)` returns one of the tied values, and which one depends on the
hash order. Put the whole rule into the key instead. Tuples compare element by element, so
`(count, Reverse(value))` compares the counts first, and for equal counts the smaller value has the
larger `Reverse`. Writing `-value` instead of `Reverse(value)` looks equivalent, but negating
`i32::MIN` overflows.

### Alternatives

Without a `HashMap`: in a sorted copy, equal values are next to each other, and `chunk_by`
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
