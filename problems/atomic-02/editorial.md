The atomic flag `locked` never races by itself, but the value inside the `UnsafeCell` is ordinary
memory. When thread B takes the lock after thread A, A's writes to the value must **happen before**
B's accesses, otherwise the two accesses are a data race, which is UB. `Ordering::Relaxed` makes each
operation on `locked` atomic but orders nothing else: B can see `locked == false` without seeing what
A wrote before unlocking. Miri tracks happens-before and reports the race where `deref_mut` creates
the `&mut T` (creating a reference counts as an access).

The fix is the classic pair of orderings:

- Unlocking is a **Release** store: everything the thread did in its critical section is published
  together with the `false`.
- Locking is an **Acquire** operation that reads that `false`: everything the Release published
  becomes visible to the new holder.

The second bug is in `try_lock`: `load`, then `store`, are two separate steps ("test, then set"). Two
threads can both load `false` before either stores `true`, and both get a guard. Stronger orderings
do not close this gap; replacing every `Relaxed` with `SeqCst` still leaves it. The test and the set
must be **one** atomic read-modify-write: `compare_exchange(false, true, Acquire, Relaxed)` changes
`false` to `true` only if the value is `false` at that very moment, and reports whether it did. The
success ordering `Acquire` is the one that matters. On failure the thread did not get the lock and
touches nothing protected, so `Relaxed` is enough.

Pitfalls:

- Acquire applies to reads and Release to writes. `store(true, Acquire)` and `load(Release)` panic at
  run time ("there is no such thing as an acquire store"), and so does a `Release` or `AcqRel`
  failure ordering in `compare_exchange`.
- `compare_exchange_weak` may fail even when the value is `false` (a spurious failure). That is fine
  in the retry loop of `lock`, but a `try_lock` built on it can return `None` for a free lock. Miri
  simulates spurious failures often.
- `SeqCst` also works once the test and the set are one operation, but it is more than a lock needs:
  Acquire when locking and Release when unlocking, which is where these names come from.

The two `unsafe impl Sync` lines were already right. `SpinLock<T>` gives `&mut T` to one thread at a
time, so, like `Mutex<T>`, it may be shared between threads when `T: Send`. A shared `&SpinGuard`
gives `&T` to every thread that has it, so the guard may be shared only when `T: Sync`. Without that
line the guard would be `Sync` whenever `T: Send`, because it only holds a `&SpinLock<T>`.

### Alternatives

`swap(true, Acquire)` also tests and sets in one step: it writes `true` and returns the old value. If
the old value was already `true`, nothing has changed.

```rust
pub fn try_lock(&self) -> Option<SpinGuard<'_, T>> {
    if self.locked.swap(true, Ordering::Acquire) {
        None
    } else {
        Some(SpinGuard { lock: self })
    }
}
```

`lock` can wait with plain loads and try the read-modify-write only when the lock looks free ("test
and test-and-set"). On real hardware, this keeps the waiting threads from fighting over the cache
line. `compare_exchange_weak` is fine here, because a spurious failure only costs one more round of
the loop. The load in the inner loop may be `Relaxed`: the Acquire that matters is the one of the
successful `compare_exchange_weak`.

```rust
pub fn lock(&self) -> SpinGuard<'_, T> {
    while self
        .locked
        .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        while self.locked.load(Ordering::Relaxed) {
            hint::spin_loop();
        }
    }
    SpinGuard { lock: self }
}
```
