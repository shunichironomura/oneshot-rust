Write a future by hand, without `async`, and a tiny executor that runs futures to completion. The
skeleton below is already in the editor; choose the fields of `Countdown` and `Delay` yourself.

```rust
pub struct Countdown { /* private */ }
pub fn countdown(n: u32) -> Countdown;
impl Future for Countdown { type Output = u32; /* poll */ }

pub struct Delay { /* private */ }
impl Delay {
    pub fn new(polls: u32) -> Self;
    pub fn extra(self, k: u32) -> Self;
}
impl IntoFuture for Delay { type Output = u32; type IntoFuture = Countdown; /* into_future */ }

pub fn block_on<F: IntoFuture>(f: F) -> F::Output;
pub fn poll_count<F: Future>(fut: F) -> (F::Output, usize);
```

- `countdown(n)` returns a future that returns `Poll::Pending` on its first `n` polls and
  `Poll::Ready(n)` on the next one. It is not polled again after it is ready.
- Each time a `Countdown` returns `Pending`, it first calls `wake_by_ref()` exactly once on the waker
  of the `Context` passed to that `poll` call. It does not wake anything when it returns `Ready`.
- `Delay` is a builder. `Delay::new(polls)` starts with `polls`, and `extra(k)` adds `k` to it; it
  can be called any number of times. `into_future()`, which `.await` calls, returns a `Countdown`
  that behaves like `countdown` of the total: awaiting `Delay::new(2).extra(1)` is like awaiting
  `countdown(3)`. The total always fits in a `u32`.
- `block_on(f)` turns `f` into a future and polls it on the current thread, over and over, with a
  waker that does nothing, until it is ready. It returns the output.
- `poll_count(fut)` runs `fut` in the same way and returns its output together with the number of
  times `poll` was called on `fut`, including the last call that returned `Ready`.
- `block_on` and `poll_count` must accept any future, including `async` blocks.

### Examples

- `poll_count(countdown(3))` is `(3, 4)`: three `Pending`s, then `Ready(3)`.
- `block_on(Delay::new(2).extra(1))` is `3`.
- `block_on(async { countdown(2).await + Delay::new(1).await })` is `3`.
- `poll_count(async { 5 })` is `(5, 1)`.
