A temporary lives until the end of its *temporary scope*, and the scrutinee of a `match` is not one:
its temporaries are dropped only after the whole `match` has finished. In the template, the guard
returned by `self.cache.lock()` is such a temporary. `.copied()` copies the value out, but the guard
itself stays alive in every arm, so the cache is still locked when the `None` arm calls
`f(self, key)`. `fib` immediately calls `get_or_compute` again, and its `lock()` waits forever for
the lock that the same thread already holds: `std::sync::Mutex` is not reentrant. (Even without
recursion, the `insert` in that arm locks a second time.)

`while let` follows the same rule: the temporaries of its condition live through the loop body. In
`run_all`, the queue stays locked while a key is processed, so `self.schedule(key / 2)` deadlocks,
and so would any call to `schedule` made by `f`.

The fix is to let each guard die in a statement of its own. In
`let cached = self.cache.lock().unwrap().get(&key).copied();` the guard is dropped at the `;`, and
`cached` is a plain `Option<u64>`. Then `f` runs while no lock is held, which is required anyway,
because `f` may call back into the memo. Locking again afterwards to insert the value is fine. In
`run_all`, a `let ... else` statement pops one key, so the queue is locked only for the `pop`.

Some fixes that look right are not:

- `*cache.entry(key).or_insert_with(|| f(self, key))` is idiomatic, but it calls `f` while the guard
  is alive. So does keeping a named guard (`let mut cache = self.cache.lock().unwrap();`) across the
  call to `f`.
- In `run_all`, keeping the queue guard in a variable and pushing `key / 2` through it avoids the
  second `lock()` and passes the examples, but `f` may call `schedule`, which still deadlocks.
- Taking the whole queue out at the start (with `std::mem::take`) and working on the taken `Vec`
  loses the keys that `f` schedules. Taking the queue out again whenever that `Vec` runs empty does
  not lose them, but processes them too late, after keys that were scheduled before them: that is
  not last in, first out.

Because the lock is released between the lookup and the insert, two threads may compute the same key
at the same time. That is the price of never holding a lock while running someone else's code, and
why the statement allows it.

### Alternatives

In Rust 2024, the temporaries of an `if let` condition are dropped before the `else` block, so the
guard can stay in the condition. In Rust 2021 this version deadlocked too, because the guard lived
until the end of the whole `if let ... else`.

```rust
pub fn get_or_compute(&self, key: u64, f: impl FnOnce(&Memo, u64) -> u64) -> u64 {
    if let Some(&value) = self.cache.lock().unwrap().get(&key) {
        value
    } else {
        let value = f(self, key);
        self.cache.lock().unwrap().insert(key, value);
        self.misses.fetch_add(1, Ordering::Relaxed);
        value
    }
}
```

A small private helper keeps the guard inside a function of its own, so the `while let` loop can
stay as it was.

```rust
fn pop(&self) -> Option<u64> {
    self.queue.lock().unwrap().pop()
}

pub fn run_all(&self, f: impl Fn(&Memo, u64) -> u64) -> Vec<(u64, u64)> {
    let mut done = Vec::new();
    while let Some(key) = self.pop() {
        let value = self.get_or_compute(key, &f);
        done.push((key, value));
        if key > 1 {
            self.schedule(key / 2);
        }
    }
    done
}
```
