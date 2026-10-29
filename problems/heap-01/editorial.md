`BinaryHeap` is a **max-heap**: `pop` and `peek` return the greatest element according to `Ord`.
Tuples compare element by element from the left, so a tuple lists the keys in order of importance.
In `(severity, Reverse(arrival), name)`, a higher severity wins first. Among equal severities,
`Reverse` makes the **smaller** arrival number the greater one, so the earliest patient comes out
first. Arrival numbers are all different, so the name is never actually compared; it only travels
along with its keys.

The trap is the tie-break. With the starting field `(u8, String)`, equal severities fall back to
comparing names, and the alphabetically last name comes out first. `Reverse(name)` passes the
example only because `bob` arrived before `cid` and also sorts before it. The arrival order has to be
stored: keep a counter that only grows. `self.heap.len()` is no substitute, because it shrinks after
`next`, and arrival numbers would repeat.

`peek` hands out a `&str` borrowed from the top element: `self.heap.peek()` gives
`Option<&(u8, Reverse<u64>, String)>`, and `name.as_str()` borrows from it for as long as `&self`.

If you scan a `Vec` instead of using a heap, note that `max_by_key` returns the **last** of several
equal maxima, so it would pick the latest of equally severe patients.

### Alternatives

A `BTreeMap` keeps its keys sorted, so the smallest key `(Reverse(severity), arrival)` belongs to the
patient to serve next. `pop_first` and `first_key_value` remove it or look at it.

```rust
use std::collections::BTreeMap;

pub struct Triage {
    waiting: BTreeMap<(Reverse<u8>, u64), String>,
    arrivals: u64,
}

impl Triage {
    pub fn new() -> Self {
        Triage { waiting: BTreeMap::new(), arrivals: 0 }
    }

    pub fn arrive(&mut self, name: &str, severity: u8) {
        self.waiting.insert((Reverse(severity), self.arrivals), name.to_string());
        self.arrivals += 1;
    }

    pub fn next(&mut self) -> Option<String> {
        self.waiting.pop_first().map(|(_, name)| name)
    }

    pub fn peek(&self) -> Option<&str> {
        self.waiting.first_key_value().map(|(_, name)| name.as_str())
    }

    pub fn len(&self) -> usize {
        self.waiting.len()
    }
}
```
