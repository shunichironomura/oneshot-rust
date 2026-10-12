A `Timeline` records events. Each event has a time `t` (a `u32`) and a short description, and there
is at most one event per time. The struct is already in the editor: it keeps the events in a
`BTreeMap`, which stores its keys in sorted order. `new` and `len` are
implemented already; implement the other methods.

```rust
use std::collections::BTreeMap;

pub struct Timeline {
    events: BTreeMap<u32, String>,
}

impl Timeline {
    pub fn new() -> Self;
    pub fn add(&mut self, t: u32, what: &str) -> Option<String>;
    pub fn len(&self) -> usize;
    pub fn between(&self, from: u32, to: u32) -> Vec<(u32, &str)>;
    pub fn last_before(&self, t: u32) -> Option<(u32, &str)>;
    pub fn split_from(&mut self, t: u32) -> Timeline;
}
```

- `new` returns an empty timeline, and `len` returns the number of events (both given).
- `add` adds the event `what` at time `t` and returns `None`. If there is already an event at `t`,
  it is replaced, and its old description is returned as `Some(old)`.
- `between` returns every event with `from <= time <= to` (both ends included) as
  `(time, description)` pairs, sorted by time in ascending order. If `from > to`, the result is
  empty.
- `last_before` returns the latest event whose time is **strictly less than** `t`, or `None` if
  there is no such event.
- `split_from` moves every event with time `>= t` out of `self` into a new timeline and returns it.
  `self` keeps the events with time `< t`.
- Every `u32` value, including `0` and `u32::MAX`, can be the time of an event or an argument.

### Examples

Each example starts from a timeline with three events: `5` "wake", `10` "work" and `20` "sleep".

- `len()` is `3`, and `between(6, 20)` is `[(10, "work"), (20, "sleep")]`.
- `last_before(10)` is `Some((5, "wake"))`, and `last_before(100)` is `Some((20, "sleep"))`.
- `add(10, "lunch")` returns `Some("work".to_string())`, and `len()` stays `3`.
- `split_from(10)` leaves only the event at `5` and returns a timeline with the events at `10` and
  `20`.
