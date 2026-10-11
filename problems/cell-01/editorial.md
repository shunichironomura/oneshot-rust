A shared reference `&T` promises that the memory behind it does not change while the reference is
in use. The only exception is memory inside an `UnsafeCell`. This rule is about `&` itself, not
about whether some `&mut` exists, so the template's `SAFETY` comment misses the point: writing
through `&raw const self.hits as *mut u32` is UB even if no other reference to the counter exists.
The compiler is allowed to rely on the rule, for example by keeping `self.hits` in a register across
a call. Miri says the same thing: `&raw const self.hits` creates the pointer from `&self`, so its tag
"only grants SharedReadOnly permission", and casting it to `*mut` does not change that.

The fix is **interior mutability**: types that wrap an `UnsafeCell` and offer a safe API for changing
their contents through `&self`. Which one fits depends on what has to come back out.

- `Cell<u32>` for the counters. `get` copies the value out and `set` replaces it. A `Cell` never
  hands out a reference to its contents, so changing them through `&self` is safe.
- The cached list is different: `keys` returns a plain `&[String]` that borrows from `&self`, and
  the caller may keep it while it calls `get` or `keys` again. A `Cell` cannot lend its contents
  through `&self` at all. A `RefCell` lends them only through a guard: in
  `self.keys.borrow().as_deref().unwrap()`, the slice borrows from the `Ref`, a temporary dropped at
  the end of `keys`, so it is rejected with E0515 ("cannot return value referencing temporary
  value"). This is not a limitation to work around: if a plain reference could outlive the guard, a
  later `borrow_mut` through another `&Lookup` could replace the vector while the caller is still
  reading it.
- `OnceCell<Vec<String>>` is the cell that fits. Through `&self` it can only go from empty to full;
  it never changes or drops a value it holds. So references to its contents stay valid as long as
  the `&self` they came from, and `get_or_init` returns one: it runs the closure only while the cell
  is empty, stores the result and returns a reference to it (`&Vec<String>`, which coerces to
  `&[String]`). The closure runs exactly when the list is sorted, so it is the place to count
  `sorts`.
- `insert` takes `&mut self`, so it needs no tricks: it replaces the cell with an empty
  `OnceCell::new()` (`self.keys.take()` works too). The borrow checker makes sure that no slice
  returned by an earlier `keys` is still in use at that point.

Changing `get` and `keys` to `&mut self` would allow plain assignments, but it breaks the callers:
two `&Lookup` at once, or an `Rc<Lookup>`, only give shared access, and with `keys(&mut self)` no
other method could be called while its slice is in use. Also take care not to use an empty list as
"not sorted yet": an empty table has an empty list, and it must still be sorted only once.

`Cell` and `OnceCell` make `Lookup` not `Sync`, so it cannot be shared between threads. That is fine
here.

### Alternatives

To share a `Lookup` between threads, use the thread-safe counterparts: `AtomicU32` for the counters
and `OnceLock<Vec<String>>` for the cache. These are also built on `UnsafeCell`.

```rust
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: AtomicU32,
    misses: AtomicU32,
    keys: OnceLock<Vec<String>>,
    sorts: AtomicU32,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: AtomicU32::new(0),
            misses: AtomicU32::new(0),
            keys: OnceLock::new(),
            sorts: AtomicU32::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        self.keys.take();
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

    pub fn keys(&self) -> &[String] {
        self.keys.get_or_init(|| {
            self.sorts.fetch_add(1, Ordering::Relaxed);
            let mut keys: Vec<String> = self.table.keys().cloned().collect();
            keys.sort_unstable();
            keys
        })
    }

    pub fn sorts(&self) -> u32 {
        self.sorts.load(Ordering::Relaxed)
    }
}
```

With `unsafe`, the cache can stay an `Option<Vec<String>>` inside an `UnsafeCell`: this is roughly
what `OnceCell` does. The rule that makes it sound is the one above: through `&self`, the cache is
written only while it is `None`, and then no slice into it can exist yet. (`Cell::update` is a short
way to add one to a counter.)

```rust
use std::cell::{Cell, UnsafeCell};
use std::collections::HashMap;

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: Cell<u32>,
    misses: Cell<u32>,
    keys: UnsafeCell<Option<Vec<String>>>,
    sorts: Cell<u32>,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: Cell::new(0),
            misses: Cell::new(0),
            keys: UnsafeCell::new(None),
            sorts: Cell::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        *self.keys.get_mut() = None;
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

    pub fn keys(&self) -> &[String] {
        let cache = self.keys.get();
        // SAFETY: through `&self`, the cache is written only while it is `None`, when no slice
        // into it exists. After that it never changes until `insert`, which needs `&mut self`.
        unsafe {
            if (*cache).is_none() {
                let mut keys: Vec<String> = self.table.keys().cloned().collect();
                keys.sort_unstable();
                *cache = Some(keys);
                self.sorts.update(|n| n + 1);
            }
            (*cache).as_deref().unwrap()
        }
    }

    pub fn sorts(&self) -> u32 {
        self.sorts.get()
    }
}
```
