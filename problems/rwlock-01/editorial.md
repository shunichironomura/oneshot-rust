A read guard is exactly the view the statement describes. `RwLockReadGuard<'_, Settings>`
dereferences to the settings, any number of read guards may exist at the same time (on one thread
or on several), and no writer gets in while one exists. So `read` simply returns
`self.settings.read().unwrap()`. The guard borrows the lock, so it cannot outlive `&self`; the
`+ '_` in the signature says so (in edition 2024 the returned `impl Trait` captures that lifetime
even without it). `update` takes the write guard, which waits until every read guard is gone.
`f(&mut settings)` works because the write guard dereferences mutably to `Settings`.

A `Mutex` compiles too, since `MutexGuard` also implements `Deref`. But a mutex has one owner at a
time: the second `read()` on the same thread waits for the first guard, which that thread will
never drop. The example in the statement already deadlocks, and Miri reports "the evaluated program
deadlocked". A mutex is not reentrant, and it would also make readers on different threads wait for
each other.

`try_update` must not wait. `try_write` returns a `TryLockResult`, and
`Err(TryLockError::WouldBlock)` means that a view or another update holds the lock, which becomes
`None`. Calling `write()` instead would wait forever for a view held by the same thread, and
`try_write().unwrap()` panics on `WouldBlock`. Get the guard first and call `f` only once you
have it; otherwise `f` runs even when the update fails. `Poisoned` only happens after a panic while
the lock was held, which cannot happen here because `f` never panics.

The version is an `AtomicU64` next to the lock, so `version()` can read it without locking. It is
increased while the write guard is still alive, so the version and the settings change together: a
thread that holds a view sees the version that belongs to it. `fetch_add` returns the *previous*
value, hence the `+ 1`. `Relaxed` is enough, because the lock already puts the updates in order.

The harness shares `&Store` between threads, so `Store` must be `Sync`. With `RefCell` or `Cell`
the harness does not compile; `RwLock<Settings>` and `AtomicU64` are `Sync`.

One more caveat: std's `RwLock` does not promise that a thread can take a second read lock. On
some platforms (Linux included), a new `read()` waits as soon as a writer is waiting. A thread
that holds a view and asks for another one while some thread waits in `update` then deadlocks,
which is why the harness never does that. Avoid nesting read locks in real code.

### Alternatives

Keep the settings and the version under one lock. Then the read guard dereferences to the whole
state rather than to `Settings`, and `&self.state.read().unwrap().settings` does not compile: it
borrows from a guard that is dropped when `read` returns. (`RwLockReadGuard::map` would do it, but
it is not stable yet.) A small wrapper that owns the guard and implements `Deref` solves this.

```rust
use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub theme: String,
    pub font_size: u32,
}

struct State {
    settings: Settings,
    version: u64,
}

pub struct Store {
    state: RwLock<State>,
}

/// A read guard of the whole state that only shows the settings.
struct View<'a>(RwLockReadGuard<'a, State>);

impl Deref for View<'_> {
    type Target = Settings;

    fn deref(&self) -> &Settings {
        &self.0.settings
    }
}

impl Store {
    pub fn new(initial: Settings) -> Store {
        Store { state: RwLock::new(State { settings: initial, version: 1 }) }
    }

    pub fn read(&self) -> impl Deref<Target = Settings> + '_ {
        View(self.state.read().unwrap())
    }

    pub fn update(&self, f: impl FnOnce(&mut Settings)) -> u64 {
        apply(self.state.write().unwrap(), f)
    }

    pub fn try_update(&self, f: impl FnOnce(&mut Settings)) -> Option<u64> {
        match self.state.try_write() {
            Ok(state) => Some(apply(state, f)),
            Err(TryLockError::WouldBlock) => None,
            Err(TryLockError::Poisoned(e)) => panic!("{e}"),
        }
    }

    pub fn version(&self) -> u64 {
        self.state.read().unwrap().version
    }

    pub fn snapshot(&self) -> Settings {
        self.state.read().unwrap().settings.clone()
    }
}

fn apply(mut state: RwLockWriteGuard<'_, State>, f: impl FnOnce(&mut Settings)) -> u64 {
    f(&mut state.settings);
    state.version += 1;
    state.version
}
```

`try_write().ok()?` is a shorter way to write `try_update` in the model solution. It also turns a
poisoned lock into `None`, which does not matter here because `f` never panics.

```rust
pub fn try_update(&self, f: impl FnOnce(&mut Settings)) -> Option<u64> {
    let guard = self.settings.try_write().ok()?;
    Some(self.apply(guard, f))
}
```
