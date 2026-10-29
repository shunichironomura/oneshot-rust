Give the function exactly the bounds its body needs, no more and no less.

The body hands every scoped thread the same closure through `let f = &f;`. What crosses threads is
the shared reference `&F`, never `F` itself, and `&F` is `Send` exactly when `F` is `Sync` (that is
what `Sync` means). So the bound is `F: Sync`, not `F: Send`. Several threads may call `f` through
`&F` at the same time, so it must be `Fn`: calling an `FnMut` needs `&mut F` (E0596). Each item is
moved into its thread (`T: Send`), and each result is moved back by `join` (`U: Send`). Nothing
needs `'static`, because `thread::scope` joins every thread before it returns. The harness relies
on that: its closures borrow `calls` and `prefix`, and in one case the item and the result borrow
`text`.

Extra bounds "just to be safe" break the harness. `F: Send + Sync` rejects the closure that owns a
`MutexGuard`: a lock must be released on the thread that took it, so a guard is `Sync` but not
`Send`. That closure never leaves the calling thread, which drops it at the end of `par_map`. The
other way around, a `Receiver` and a `Cell` are `Send` but not `Sync`, so `T: Sync` rejects the
`Receiver` items, and `U: Sync` rejects the closure that returns a `Cell`. `'static` rejects the
closures and items that borrow local variables.

### Alternatives

`impl Trait` in argument position works too.

```rust
use std::thread;

pub fn par_map<T: Send, U: Send>(items: Vec<T>, f: impl Fn(T) -> U + Sync) -> Vec<U> {
    thread::scope(|s| {
        let f = &f;
        let handles: Vec<_> = items.into_iter().map(|x| s.spawn(move || f(x))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}
```

Since `F: Sync` means "`&F` is `Send`", you can also write it that way, with a higher-ranked bound
over the lifetime of the reference.

```rust
pub fn par_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send,
    U: Send,
    F: Fn(T) -> U,
    for<'a> &'a F: Send,
```
