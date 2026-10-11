A `Lock` is a named resource that writes to a shared log when it is acquired and when it is
released. Dropping a `Lock` releases it. Implement `Lock` and four functions that use it.

```rust
pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Lock {
    name: String,
    log: Log,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock;
    pub fn name(&self) -> &str;
}

impl Drop for Lock { /* to implement */ }

pub fn with_lock<R>(name: &str, log: &Log, f: impl FnOnce() -> R) -> R;
pub fn release_all(locks: Vec<Lock>);
pub fn swap_in(slot: &mut Lock, name: &str);
pub fn keep_first(locks: Vec<Lock>) -> Option<Lock>;
```

- `acquire` creates a lock named `name` that writes to `log`, and appends `"acquire <name>"` to
  `log`.
- When a lock is dropped, it appends `"release <name>"` to its log.
- `name` returns the name of the lock. A name can be any string, even the empty string.
- `with_lock` acquires a lock named `name` that writes to `log`, calls `f` while it holds the lock,
  then releases the lock and returns what `f` returned. So whatever `f` appends to `log` comes
  after `"acquire <name>"` and before `"release <name>"`.
- `release_all` releases every lock in `locks`, from the **last** one to the first.
- `swap_in` replaces the lock in `slot` with a new lock named `name` that writes to the same log.
  The new lock is acquired first, and then the old one is released.
- `keep_first` returns the first lock of `locks` (`None` if `locks` is empty) and releases all the
  other locks, in their order in `locks`.

The code above is already in the editor. Do not change `Log` or the public signatures.

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
// first.unwrap().name() == "a"
```

```rust
let n = with_lock("m", &log, || {
    log.borrow_mut().push("work".to_string());
    42
});
// n == 42, and the call appended "acquire m", "work", "release m"
```
