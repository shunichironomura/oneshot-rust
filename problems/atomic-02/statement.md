This spin lock compiles, and when you run it natively it seems to work. But it contains
**undefined behavior (UB)**, and Miri detects it.

`SpinLock<T>` protects a value of type `T` so that only one thread at a time can use it. Unlike
`std::sync::Mutex`, it never puts a waiting thread to sleep: it keeps trying until the lock is free
(it "spins"). The whole implementation is already in the editor.

```rust
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self;
    pub fn lock(&self) -> SpinGuard<'_, T>;
    pub fn try_lock(&self) -> Option<SpinGuard<'_, T>>;
    pub fn into_inner(self) -> T;
}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

// SpinGuard<'_, T> implements Deref<Target = T>, DerefMut and Drop.
```

- A guard gives access to the value: `*guard` reads it, and `*guard = ...` or a method call such as
  `guard.push_str(...)` changes it. At most one guard of a lock is alive at any time, even when
  several threads call `lock` and `try_lock` at the same time.
- A change made through a guard is seen through every later guard of the same lock, whichever thread
  holds it.
- `lock` waits until the lock is free, takes it and returns a guard. It calls
  `std::hint::spin_loop()` between attempts.
- `try_lock` never waits. If the lock is free at the moment of the call, it takes it and returns
  `Some(guard)`. It returns `None` only while a guard of the lock is alive (in any thread, including
  the calling one).
- Dropping a guard frees the lock.
- `new` is a `const fn`, so a `SpinLock` can be stored in a `static`.
- `into_inner` consumes the lock and returns the value.

Remove the UB so that the lock works as described. Keep the fields of `SpinLock` and `SpinGuard`,
and do not use the locks of `std::sync` (such as `Mutex`). (You still need `unsafe` to reach the
value inside the `UnsafeCell`.)

### Examples

```rust
let counter = SpinLock::new(0);
thread::scope(|s| {
    for _ in 0..3 {
        s.spawn(|| {
            for _ in 0..5 {
                *counter.lock() += 1;
            }
        });
    }
});
assert_eq!(counter.into_inner(), 15);

let lock = SpinLock::new(String::from("spin"));
let mut guard = lock.lock();
guard.push_str("lock");
assert!(lock.try_lock().is_none()); // the guard is still alive
drop(guard);
assert_eq!(lock.try_lock().map(|g| g.len()), Some(8));
```
