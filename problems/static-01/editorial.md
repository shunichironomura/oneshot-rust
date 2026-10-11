Two of the three statics are rejected, each for its own reason. `NAMES` is fine as written.

- **A static's initializer is evaluated at compile time**, so it may only call `const fn`s.
  `Mutex::new`, `RwLock::new` and `Vec::new` are `const fn`, but `HashMap::new` is not: it creates a
  `RandomState`, which picks random hash keys when the program runs (E0015). `LazyLock` moves the
  initialization to run time: the closure runs once, on the first use from whichever thread gets
  there first, and the static derefs to the value, so `IDS.lock()` works as before.
- **A static must be `Sync`**, because every thread can reach it at the same time. `Cell` is not
  `Sync` (E0277): `CALLS.set(CALLS.get() + 1)` on two threads at once would be a data race.
  `AtomicUsize::fetch_add` increments in one indivisible step. `Ordering::Relaxed` is enough because
  the counter does not guard any other data, and `join` makes a thread's increments visible to the
  thread that joins it.

Neither `static mut` nor `thread_local!` is a way out. `static mut` needs `unsafe` (forbidden here,
and Rust 2024 even denies references to it). `thread_local!` compiles, but then every thread has its
own table and counter: ids from different threads clash, and `intern_calls` counts only the calls of
the current thread.

The rest of `intern` was already right and should stay that way. `IDS` stays locked from the lookup
until the new string is in both tables, so two threads cannot give one string two ids, and the next
id is always `names.len()`. Locking `IDS` once for the lookup and again for the insert would let two
threads both miss the lookup. The locks are always taken in the order `IDS`, then `NAMES`, so they
cannot deadlock.

### Alternatives

`BTreeMap::new` is a `const fn`, so a `BTreeMap` (from `std::collections`) needs no `LazyLock`.
`HashMap::with_hasher` is a `const fn` too, with a hasher that does not pick random keys, such as
`BuildHasherDefault::<DefaultHasher>::new()`.

```rust
static IDS: Mutex<BTreeMap<String, u32>> = Mutex::new(BTreeMap::new());
```

`OnceLock` is the manual form of `LazyLock`: the initializer is given where the value is used.

```rust
static IDS: OnceLock<Mutex<HashMap<String, u32>>> = OnceLock::new();

pub fn intern(s: &str) -> u32 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    let table = IDS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut ids = table.lock().unwrap();
    // ... the rest as before
}
```

With one `Mutex` around all of the state, no thread can ever see it half updated, and there is no
lock order to get wrong. The price is that readers wait for each other, which the `RwLock` avoided.

```rust
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Default)]
struct Interner {
    ids: HashMap<String, u32>,
    names: Vec<String>,
    calls: usize,
}

static INTERNER: LazyLock<Mutex<Interner>> = LazyLock::new(|| Mutex::new(Interner::default()));

pub fn intern(s: &str) -> u32 {
    let mut table = INTERNER.lock().unwrap();
    table.calls += 1;
    if let Some(&id) = table.ids.get(s) {
        return id;
    }
    let id = table.names.len() as u32;
    table.names.push(s.to_string());
    table.ids.insert(s.to_string(), id);
    id
}

pub fn resolve(id: u32) -> Option<String> {
    INTERNER.lock().unwrap().names.get(id as usize).cloned()
}

pub fn interned_count() -> usize {
    INTERNER.lock().unwrap().names.len()
}

pub fn intern_calls() -> usize {
    INTERNER.lock().unwrap().calls
}
```
