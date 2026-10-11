A `Lock` is a named resource that writes to a shared log when it is acquired and when it is
released. Dropping a `Lock` releases it. Implement `Lock` and three functions that use it.

```rust
pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Lock {
    name: String,
    log: Log,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock;
    pub fn name(&self) -> &str;
    pub fn into_name(self) -> String;
}

impl Drop for Lock { /* to implement */ }

pub fn release_all(locks: Vec<Lock>);
pub fn swap_in(slot: &mut Lock, name: &str);
pub fn keep_first(locks: Vec<Lock>) -> Option<Lock>;
```

- `acquire` creates a lock named `name` that writes to `log`, and appends `"acquire <name>"` to
  `log`.
- When a lock is dropped, it appends `"release <name>"` to its log.
- `name` returns the name of the lock. A name can be any string, even the empty string.
- `into_name` consumes the lock and returns its name **without** releasing it: nothing is appended
  to the log, now or later.
- `release_all` releases every lock in `locks`, from the **last** one to the first.
- `swap_in` replaces the lock in `slot` with a new lock named `name` that writes to the same log.
  The new lock is acquired first, and then the old one is released.
- `keep_first` returns the first lock of `locks` (`None` if `locks` is empty) and releases all the
  other locks, in their order in `locks`.
- A lock that has been released or consumed by `into_name` must not keep its log alive: once every
  lock that writes to `log` is gone, `Rc::strong_count(&log)` is `1` again.

The code above is already in the editor. Do not change `Log` or the public signatures. You may
change the private fields of `Lock`.

### Examples

```rust
let log: Log = Rc::new(RefCell::new(Vec::new()));
let a = Lock::acquire("a", &log);
let b = Lock::acquire("b", &log);
let c = Lock::acquire("c", &log);
release_all(vec![a, b, c]);
// log: ["acquire a", "acquire b", "acquire c", "release c", "release b", "release a"]
```

```rust
let mut x = Lock::acquire("x", &log); // appends "acquire x"
swap_in(&mut x, "y");                 // appends "acquire y", "release x"
// x.name() == "y"
```

```rust
let a = Lock::acquire("a", &log);
let b = Lock::acquire("b", &log);
let c = Lock::acquire("c", &log);
let first = keep_first(vec![a, b, c]); // appends "release b", "release c"
let a = first.unwrap();                // a.name() == "a"
a.into_name();                         // returns "a" and appends nothing
```
