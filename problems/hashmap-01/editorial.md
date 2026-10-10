The `entry` API lets you insert-or-update in one lookup: `*counts.entry(key).or_insert(0) += 1`.
`or_default()` works too, since `usize::default()` is `0`.

For the order, compare by count in **reverse** (`nb.cmp(na)`) and break ties with `then_with`.

### Alternatives

Count into a `BTreeMap`, whose iteration is already alphabetical. `sort_by` is a stable sort, so
sorting only by count keeps words with the same count in alphabetical order.

```rust
use std::collections::BTreeMap;

pub fn word_count(text: &str) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w.to_lowercase()).or_default() += 1;
    }
    let mut result: Vec<(String, usize)> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}
```

`sort_by_key(|(w, n)| (std::cmp::Reverse(*n), w.clone()))` also works, but it clones a `String` for
every comparison.
