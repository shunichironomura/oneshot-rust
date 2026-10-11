Rust has no per-type statics, so the counters live in **one** `static` map from the type to its
count. `TypeId::of::<T>()` identifies a type, and it is why every function requires `T: 'static`:
lifetimes are erased before the program runs, so `&'a str` and `&'static str` could not get
different ids.

- A static's initializer is evaluated at compile time, so it may only call `const fn`s.
  `Mutex::new` and `BTreeMap::new` are `const fn`, and `TypeId` implements `Ord`, so
  `Mutex<BTreeMap<TypeId, usize>>` needs no lazy initialization. `HashMap::new` is not `const`
  (E0015); see the alternatives.
- A static must be `Sync`, because every thread can reach it: `Mutex`, not `RefCell`.
- `register` increments the count and reads it back under one lock, so two threads can never get
  the same value. `count` uses `get`, not `entry(..).or_insert(0)`, so asking about a type does not
  add it to the map, which would make `distinct` count it.

**The trap** is a static declared inside a generic function, which looks like one counter per `T`:

```rust
fn counter<T: 'static>() -> &'static AtomicUsize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    &COUNT
}
```

It compiles, but there is only one `COUNT`. The compiler copies the function's code for every `T`
(monomorphization), but not its statics: a static inside a generic function is a single item, as if
it were declared outside the function. (It cannot even mention `T`; that is error E0401.) All types
share the counter, so in the first example `register::<Pear>()` returns `3`. This differs from C++,
where a `static` local variable in a function template exists once per instantiation.

Two other shortcuts fail the hidden cases:

- `std::any::type_name::<T>()` is not a unique key. Two structs named `Apple` declared in two test
  cases are different types, but both are named `main::tests::run::{{closure}}::Apple`. Its
  documentation warns that the name must not be considered a unique identifier.
- `thread_local!` compiles, but gives each thread its own counters.

### Alternatives

`HashMap` works too. `HashMap::new` is not `const` (its `RandomState` picks random keys at run
time), so here the map is created on first use with `LazyLock`. The four functions stay the same as
in the model solution.

```rust
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static COUNTS: LazyLock<Mutex<HashMap<TypeId, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
```

`HashMap::with_hasher` is `const`, though, so a hasher that needs no random keys avoids the lazy
initialization as well (the four functions again stay the same):

```rust
use std::any::TypeId;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, DefaultHasher};
use std::sync::Mutex;

type Counts = HashMap<TypeId, usize, BuildHasherDefault<DefaultHasher>>;

static COUNTS: Mutex<Counts> = Mutex::new(HashMap::with_hasher(BuildHasherDefault::new()));
```

A static can hold a struct, and a struct expression is fine in a `const` initializer. Here the total
is kept next to the map under the same lock, so `total` does not have to add up every counter.

```rust
use std::any::TypeId;
use std::collections::BTreeMap;
use std::sync::Mutex;

struct Counters {
    per_type: BTreeMap<TypeId, usize>,
    total: usize,
}

static COUNTERS: Mutex<Counters> = Mutex::new(Counters {
    per_type: BTreeMap::new(),
    total: 0,
});

pub fn register<T: 'static>() -> usize {
    let mut counters = COUNTERS.lock().unwrap();
    counters.total += 1;
    let n = counters.per_type.entry(TypeId::of::<T>()).or_insert(0);
    *n += 1;
    *n
}

pub fn count<T: 'static>() -> usize {
    let counters = COUNTERS.lock().unwrap();
    counters.per_type.get(&TypeId::of::<T>()).copied().unwrap_or(0)
}

pub fn total() -> usize {
    COUNTERS.lock().unwrap().total
}

pub fn distinct() -> usize {
    COUNTERS.lock().unwrap().per_type.len()
}
```
