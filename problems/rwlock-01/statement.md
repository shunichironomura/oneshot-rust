Many parts of an application read its settings, often at the same time and from several threads,
while changes are rare. Implement `Store`, which holds the settings. `Settings` is already in the
editor; choose the fields of `Store` yourself.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub theme: String,
    pub font_size: u32,
}

pub struct Store {
    // fields of your choice
}

impl Store {
    pub fn new(initial: Settings) -> Store;
    pub fn read(&self) -> impl Deref<Target = Settings> + '_;
    pub fn update(&self, f: impl FnOnce(&mut Settings)) -> u64;
    pub fn try_update(&self, f: impl FnOnce(&mut Settings)) -> Option<u64>;
    pub fn version(&self) -> u64;
    pub fn snapshot(&self) -> Settings;
}
```

- The harness shares one `&Store` between several threads (with `std::thread::scope`).
- The store has a version number that starts at `1`. `version()` returns it.
- `read()` returns a *view* of the current settings: `*view` is the settings, and `view.theme`
  reads a field. The settings do not change while a view is alive. Any number of views may be alive
  at the same time, on one thread or on several threads.
- `update(f)` waits until no view is alive and no other update is running. Then it calls `f` once
  with the settings, adds 1 to the version and returns the new version. When several threads update
  at the same time, no update is lost, and each one returns a different version.
- `try_update(f)` does the same as `update(f)` and returns `Some(new version)` if it can start
  right away. If a view is alive or another update is running, it returns `None` immediately,
  without calling `f` and without changing anything (the version included).
- `snapshot()` returns a copy of the current settings, which later updates do not change.
- `version()` and `snapshot()` may be called while views are alive.
- The harness never calls `update` on a thread that holds a view (it would wait forever). While a
  thread holds a view, it calls the store's methods only if no other thread is using the store.
  `f` never uses the store and never panics.

### Example

```rust
let store = Store::new(Settings { theme: "dark".to_string(), font_size: 12 });
let a = store.read();
let b = store.read();                            // two views at the same time
// a.theme == b.theme, and b.font_size == 12
store.try_update(|s| s.font_size = 14)           // None: views are alive, f is not called
drop(a);
drop(b);
store.try_update(|s| s.font_size = 14)           // Some(2)
store.update(|s| s.theme = "light".to_string())  // 3
store.version()                                  // 3
store.snapshot()                                 // Settings { theme: "light", font_size: 14 }
```
