A `std::sync::MutexGuard` is not `Send`, because some platforms require a mutex to be unlocked by
the thread that locked it. The future of an `async fn` stores every value that is still alive at an
`.await`, so a guard alive there makes the whole future non-`Send`, and the harness cannot move it
to another thread ("future cannot be sent between threads safely"). It is a real bug, too: while the
future waits, every other task that wants the lock blocks its whole thread, and a task on the same
thread deadlocks. Both functions hold a guard across an `.await`, in two different ways.

**`record_all`**: `guard` lives until the end of the loop body, after `yield_now().await`. The
obvious fix, `drop(guard);` just before the `.await`, unlocks in time but still does not compile. To
decide what the future must store, the compiler looks at which variables may still be needed after
the `.await`, and it is conservative about borrowed ones: once a variable has been borrowed
(`guard.push(item)` and `guard.len()` borrow `guard` through `DerefMut` and `Deref`), it counts as
needed until the end of its scope, since a reference to it might still exist. `drop(guard)` moves
the guard out, but the variable is still in scope at the `.await`, so a `MutexGuard` is still
counted as part of the future. Put the guard in a block that ends before the `.await`: the block
evaluates to the new length, and the guard's scope is over.

**`forward_all`**: there is no guard variable here, but `inbox.lock().unwrap()` creates a temporary
guard, and a temporary lives until the end of the statement it is in. The body is a single
expression that ends with `.await`, so `inbox` stays locked while `record_all` runs. Taking the
messages out in a `let` statement of their own drops the temporary at that `;`. Writing
`let len = record_all(log, take(...)).await;` is not enough, since the `.await` is still in the same
statement. A temporary guard in a statement without `.await`, such as
`let mut len = log.lock().unwrap().len();`, is fine.

Locking twice, `log.lock().unwrap().push(item); len = log.lock().unwrap().len();`, also compiles,
but another thread may push between the two locks, so `len` may not be the length right after this
push. Async runtimes such as Tokio provide their own mutexes, whose guards may be held across an
`.await`; a `std` guard must not be.

### Alternatives

Lock in an ordinary (non-async) function. Its guard can never be alive at an `.await`, however the
function is written.

```rust
fn push_and_len(log: &Mutex<Vec<String>>, item: String) -> usize {
    let mut guard = log.lock().unwrap();
    guard.push(item);
    guard.len()
}

pub async fn record_all(log: &Mutex<Vec<String>>, items: Vec<String>) -> usize {
    let mut len = log.lock().unwrap().len();
    for item in items {
        len = push_and_len(log, item);
        yield_now().await;
    }
    len
}
```

A block statement works as well as a block expression.

```rust
for item in items {
    {
        let mut guard = log.lock().unwrap();
        guard.push(item);
        len = guard.len();
    }
    yield_now().await;
}
```

Since edition 2024, the temporaries of a block's final expression are dropped at the end of the
block, so wrapping the argument of `forward_all` in a block is enough. (In edition 2021 they lived
until the end of the enclosing statement, and this did not compile.)

```rust
pub async fn forward_all(inbox: &Mutex<Vec<String>>, log: &Mutex<Vec<String>>) -> usize {
    record_all(log, { std::mem::take(&mut *inbox.lock().unwrap()) }).await
}
```
