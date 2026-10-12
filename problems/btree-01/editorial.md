A `BTreeMap` keeps its keys sorted, so `range` visits exactly the keys inside a range, in ascending
order. Its iterator is double-ended: `next()` gives the smallest key in the range and `next_back()`
the largest. Each of the two queries is a single `range` call.

- `between`: `range(from..=to)`.
- `last_before`: `range(..t).next_back()`, the largest key below `t`.

The trap is in the documentation of `range`: it **panics** if the start is greater than the end
(`range start is greater than range end in BTreeMap`). So `between(9, 3)` must be handled before
calling `range`. A freshly created empty map does not panic (it has no tree yet), so trying it on a
new, empty timeline does not reveal the bug.

Also avoid arithmetic on the bounds. `range(from..to + 1)` overflows when `to` is `u32::MAX`, and
`range(..=t - 1)` underflows when `t` is `0`; both panic in debug builds such as the judge's.
Saturating arithmetic hides the panic but gives wrong answers: `..=t.saturating_sub(1)` includes
time `0` when `t` is `0`. The range forms `..=to` and `..t` express the same bounds without any
arithmetic.

`split_off(&t)` moves the keys `>= t` into a new map and keeps the smaller ones in place, which is
exactly `split_from`.

The iterator yields `(&u32, &String)` pairs. Deref coercion does not reach inside a tuple, so
mapping to `(time, what)` gives a `(u32, &String)`, and collecting that into `Vec<(u32, &str)>`
does not compile; `what.as_str()` converts it explicitly. Thanks to lifetime elision, the returned
`&str` borrows from `&self`.

### Alternatives

`between` can also filter all events, which never panics, but it walks the whole map instead of only
the requested range.

```rust
pub fn between(&self, from: u32, to: u32) -> Vec<(u32, &str)> {
    self.events
        .iter()
        .filter(|&(&time, _)| from <= time && time <= to)
        .map(|(&time, what)| (time, what.as_str()))
        .collect()
}
```
