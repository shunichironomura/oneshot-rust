An async closure such as `async || { calls += 1; ... }` keeps `&mut calls` inside the closure, and
each future it returns borrows that state **from the closure**: the future is only valid while the
call that created it keeps `f` borrowed. `F: FnMut() -> Fut` cannot describe this. `Fut` is one
type for every call, so it cannot refer to a borrow of `f` that only exists during one call. That is
why the compiler says "async closure does not implement `FnMut` because it captures state from its
environment". `async move ||` does not help: the closure then owns the state, and the future still
borrows it from the closure.

Rust 1.85 added async closures together with the traits `AsyncFn`, `AsyncFnMut` and `AsyncFnOnce`
(they are in the prelude). `F: AsyncFnMut() -> Result<T, E>` means "calling `f` borrows it mutably
until the returned future has finished", which is exactly how the body uses it: each `f().await`
finishes before the next call. The other two traits do not fit:

- `AsyncFn` borrows `f` only immutably, so it rejects closures that change what they capture
  (`calls += 1`, `log.push(...)`, and the closure in `fetch_any` that advances `urls`).
- `AsyncFnOnce` allows only one call, but the body calls `f` in a loop (E0382).

Calling an `AsyncFnMut` needs `&mut f`, so the parameters stay `mut f` and `mut fetch`. The future
type is no longer named, so remove `Fut` from the type parameters too: nothing in the arguments
determines it any more, and leaving it in fails with "type annotations needed" (E0282) at every
call.

`fetch_any` needs one more thing. Its body passes `&url`, a reference to a `String` that lives only
inside the inner closure, and the future of `fetch` keeps that reference: the future of an
`async |url: &str|` closure or of `async fn probe(url: &str)` always holds its arguments. So the
future type depends on the lifetime of the argument, and `F: FnMut(&str) -> Fut`, with a single
`Fut` for every lifetime, fails for `probe` (E0271). `F: AsyncFnMut(&str) -> Result<T, E>` is short
for `F: for<'a> AsyncFnMut(&'a str) -> Result<T, E>`, and its futures may borrow the argument (only
the output `Result<T, E>` may not). A lifetime parameter chosen by the caller, as in
`AsyncFnMut(&'a str)`, does not work, because `url` is dropped inside `fetch_any` (E0597).

`AsyncFnMut` is also implemented by ordinary closures and functions that return futures, and by
`async fn`s such as `probe`, so callers of the old signature keep working. (A non-`move` async
closure that only needs shared references to what it captures, like the ones in `hidden-4`, even
satisfied the old bound: its future copies those references instead of borrowing the closure.)

### Alternatives

With `impl Trait` in argument position, the closures need no type parameter.

```rust
pub async fn retry<T, E>(times: usize, mut f: impl AsyncFnMut() -> Result<T, E>) -> Result<T, E>

pub async fn fetch_any<T, E>(
    mirrors: &[&str],
    path: &str,
    mut fetch: impl AsyncFnMut(&str) -> Result<T, E>,
) -> Result<T, E>
```

The higher-ranked bound of `fetch_any` can also be written explicitly.

```rust
pub async fn fetch_any<T, E, F>(mirrors: &[&str], path: &str, mut fetch: F) -> Result<T, E>
where
    F: for<'a> AsyncFnMut(&'a str) -> Result<T, E>,
```
