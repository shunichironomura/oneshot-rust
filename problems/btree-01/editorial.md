A `BTreeMap` keeps its keys sorted, so `range` visits exactly the keys inside a range, in ascending
order. Its iterator is double-ended: `next()` gives the smallest key in the range and `next_back()`
the largest. Each of the three queries is a single `range` call.

- `between`: `range(from..=to)`.
- `last_before`: `range(..t).next_back()`, the largest key below `t`.
- `next_at_or_after`: `range(t..).next()`, the smallest key from `t` on.

The trap is in the documentation of `range`: it **panics** if the start is greater than the end
(`range start is greater than range end in BTreeMap`). So `between(9, 3)` must be handled before
calling `range`. The panic only happens when the map is not empty, which makes it easy to miss
when you try it on an empty timeline.

Also avoid arithmetic on the bounds. `range(from..to + 1)` overflows when `to` is `u32::MAX`, and
`range(..=t - 1)` underflows when `t` is `0`; both panic in debug builds such as the judge's. The
range forms `..=to` and `..t` express the same bounds without any arithmetic.

`split_off(&t)` moves the keys `>= t` into a new map and keeps the smaller ones in place, which is
exactly `split_from`. `prune_before` wants the other half: keep what `split_off` returns and swap it
in with `std::mem::replace`, which hands back the removed part so you can count it.

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

`prune_before` with `retain`, counting the events before and after.

```rust
pub fn prune_before(&mut self, t: u32) -> usize {
    let before = self.events.len();
    self.events.retain(|&time, _| time >= t);
    before - self.events.len()
}
```
