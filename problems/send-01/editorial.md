Give each function exactly the bounds its body needs. The two bodies use `f` in opposite ways, so
the template's `FnMut` is wrong for both, in opposite directions.

**`par_map`** hands every scoped thread the same closure through `let f = &f;`. What crosses threads
is the shared reference `&F`, never `F` itself, and `&F` is `Send` exactly when `F` is `Sync` (that
is what `Sync` means). So the bound is `F: Sync`, not `F: Send`. Several threads may call `f` through
`&F` at the same time, so it must be `Fn`: calling an `FnMut` needs `&mut F` (E0596). Each item is
moved into its thread (`T: Send`), and each result is moved back by `join` (`U: Send`). Nothing needs
`'static`, because `thread::scope` joins every thread before it returns. The harness relies on that:
its closures borrow `calls` and `prefix`, and in one case the item and the result borrow `text`.

**`spawn_job`** uses `thread::spawn`, whose thread may outlive the caller, so the closure
`move || f(input)` and its result must be `Send + 'static`. The closure owns `f` and `input`, hence
`F: Send + 'static` and `T: Send + 'static`, and the result comes back through the `JoinHandle<U>`,
hence `U: Send + 'static`. `f` is called only once, so `FnOnce` is enough, and the harness needs
exactly that: `move |n: usize| name + ...` gives away the `String` it captured, so it is neither
`FnMut` nor `Fn` (E0507).

Extra bounds "just to be safe" break the harness. `F: Send + Sync` in `par_map` rejects the closure
that owns a `MutexGuard`: a lock must be released on the thread that took it, so a guard is `Sync`
but not `Send`. That closure never leaves the calling thread, which drops it at the end of
`par_map`. The other way around, a `Receiver` and a `Cell` are `Send` but not `Sync`, so `F: Sync`
in `spawn_job` rejects the closure that owns a receiver. In the same way, `T: Sync` rejects the
`Receiver` items and inputs, and `U: Sync` rejects the closure that returns a `Cell` and the job
that gives its receiver back. `'static` in `par_map` rejects the closures and items that borrow
local variables.

### Alternatives

`impl Trait` in argument position works too.

```rust
use std::thread::{self, JoinHandle};

pub fn par_map<T: Send, U: Send>(items: Vec<T>, f: impl Fn(T) -> U + Sync) -> Vec<U> {
    thread::scope(|s| {
        let f = &f;
        let handles: Vec<_> = items.into_iter().map(|x| s.spawn(move || f(x))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

pub fn spawn_job<T, U>(input: T, f: impl FnOnce(T) -> U + Send + 'static) -> JoinHandle<U>
where
    T: Send + 'static,
    U: Send + 'static,
{
    thread::spawn(move || f(input))
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
