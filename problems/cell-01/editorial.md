A shared reference `&T` promises that the memory behind it does not change while the reference is
in use. The only exception is memory inside an `UnsafeCell`. This rule is about `&` itself, not
about whether some `&mut` exists, so the template's `SAFETY` comment misses the point: writing
through `&raw const self.hits as *mut u32` is UB even if no other reference to the counter exists.
The compiler is allowed to rely on the rule, for example by keeping `self.hits` in a register across
a call. Miri says the same thing: the pointer was derived from `&self`, whose tag "only grants
SharedReadOnly permission".

The fix is **interior mutability**: types that wrap an `UnsafeCell` and offer a safe API for changing
their contents through `&self`.

- `Cell<u32>` for the counters. `get` copies the value out and `set` replaces it. Through `&self`, a
  `Cell` never hands out a reference to its contents, so changing them through `&self` is safe.
- `OnceCell<u32>` for the cached sum. `get_or_init` runs the closure only while the cell is empty
  and returns a `&u32` to the stored value (hence the `*`). The closure runs exactly when the sum is
  computed, so it is the place to count `computations`.
- `insert` takes `&mut self`, so it needs no tricks: it replaces the cell with an empty
  `OnceCell::new()` (`self.total.take()` works too).

Changing `get` and `total` to `&mut self` would allow a plain `self.hits += 1`, but it breaks the
callers: two `&Lookup` at once, or an `Rc<Lookup>`, only give shared access. Also take care not to
use `0` as "not computed yet": the sum of a table can be `0`, and it must still be computed only
once.

`Cell` makes `Lookup` not `Sync`, so it cannot be shared between threads. That is fine here.

### Alternatives

`Option<u32>` is `Copy`, so a `Cell<Option<u32>>` can hold the cache as well. `Cell::update` is a
short way to add one.

```rust
use std::cell::Cell;
use std::collections::HashMap;

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: Cell<u32>,
    misses: Cell<u32>,
    total: Cell<Option<u32>>,
    computations: Cell<u32>,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: Cell::new(0),
            misses: Cell::new(0),
            total: Cell::new(None),
            computations: Cell::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        self.total.set(None);
    }

    pub fn get(&self, key: &str) -> Option<u32> {
        let value = self.table.get(key).copied();
        let counter = if value.is_some() { &self.hits } else { &self.misses };
        counter.update(|n| n + 1);
        value
    }

    pub fn hits(&self) -> u32 {
        self.hits.get()
    }

    pub fn misses(&self) -> u32 {
        self.misses.get()
    }

    pub fn total(&self) -> u32 {
        if let Some(total) = self.total.get() {
            return total;
        }
        let total = self.table.values().sum();
        self.total.set(Some(total));
        self.computations.update(|n| n + 1);
        total
    }

    pub fn computations(&self) -> u32 {
        self.computations.get()
    }
}
```

To share a `Lookup` between threads, use the thread-safe counterparts: `AtomicU32` for the counters
and `OnceLock<u32>` for the cache. These are also built on `UnsafeCell`.

```rust
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: AtomicU32,
    misses: AtomicU32,
    total: OnceLock<u32>,
    computations: AtomicU32,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: AtomicU32::new(0),
            misses: AtomicU32::new(0),
            total: OnceLock::new(),
            computations: AtomicU32::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        self.total.take();
    }

    pub fn get(&self, key: &str) -> Option<u32> {
        let value = self.table.get(key).copied();
        let counter = if value.is_some() { &self.hits } else { &self.misses };
        counter.fetch_add(1, Ordering::Relaxed);
        value
    }

    pub fn hits(&self) -> u32 {
        self.hits.load(Ordering::Relaxed)
    }

    pub fn misses(&self) -> u32 {
        self.misses.load(Ordering::Relaxed)
    }

    pub fn total(&self) -> u32 {
        *self.total.get_or_init(|| {
            self.computations.fetch_add(1, Ordering::Relaxed);
            self.table.values().sum()
        })
    }

    pub fn computations(&self) -> u32 {
        self.computations.load(Ordering::Relaxed)
    }
}
```
