Write `with_budget`, which wraps a future in a `Budget`: a future that runs the wrapped future, but
gives up if it has not finished after a given number of polls. There is no async runtime (only `std`
is available). The struct `Budget` with its two fields is already in the editor.

```rust
pub struct Budget<F> {
    inner: Option<F>,
    left: u32,
}

pub fn with_budget<F: Future>(fut: F, polls: u32) -> Budget<F>;

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;
    /* poll */
}
```

Below, `fut` is called the *inner future*, and `polls` the *budget*.

- Like the combinators of async libraries, a `Budget` keeps the inner future inside itself, so that
  wrapping a future allocates nothing. Do not change the fields, and do not move the inner future to
  the heap: `size_of::<Budget<F>>()` is at least `size_of::<F>()`. `Budget<F>` is `Unpin` exactly
  when `F` is.
- Calling `with_budget` polls nothing and drops nothing.
- If the budget is `0`, the first poll of the `Budget` returns `Poll::Ready(None)` without polling
  the inner future.
- Otherwise, each poll of the `Budget` polls the inner future once, passing on the `Context` that the
  `Budget` is being polled with. Then:
  - If the inner future returned `Poll::Ready(v)`, the `Budget` returns `Poll::Ready(Some(v))`.
  - If it returned `Poll::Pending`, the `Budget` returns `Poll::Pending`, unless that was the
    `polls`-th poll of the inner future. Then the budget is used up, and the `Budget` gives up right
    away: it returns `Poll::Ready(None)` in that same poll.
- The inner future is dropped as soon as it is no longer needed. Whenever the `Budget` returns
  `Poll::Ready`, it has already dropped the inner future during that poll, before returning; it does
  not wait until the `Budget` itself is dropped. A `Budget` that is dropped before it is ready drops
  the inner future with it.
- Once the inner future has been polled, it stays at the same place in memory until it is dropped,
  as `Pin` requires.
- The `Budget` never wakes the waker itself; waking it is up to the inner future.
- The inner future can be any future, including an `async` block that borrows local variables (also
  its own, across an `.await`) and is neither `Unpin` nor `Send`. Its output can be of any type.
- You may use `unsafe`. Undefined behavior and memory leaks are detected by Miri and fail the
  submission.

The test harness polls the futures on the current thread. Some tests run them like a real executor,
which polls a future again only after its waker has been woken. The harness never polls a `Budget`
again after it has returned `Poll::Ready`.

### Examples

In the examples, `ticks(n, out)` stands for a future of the harness that is not `Unpin`. On each of
its first `n` polls, it wakes the waker and returns `Poll::Pending`; on the next poll, it returns
`Poll::Ready(out)`.

- `with_budget(ticks(2, "done"), 3)` returns `Pending` on its first two polls and
  `Ready(Some("done"))` on the third. When the third poll returns, the `ticks` future has been
  polled three times and has already been dropped, although the `Budget` still exists.
- `with_budget(ticks(5, "late"), 3)` returns `Pending` on its first two polls and `Ready(None)` on
  the third: the `ticks` future returned `Pending` on each of its three polls, which used up the
  budget. When the third poll returns, the `ticks` future has already been dropped.
