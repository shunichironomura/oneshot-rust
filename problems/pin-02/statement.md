Write `with_budget`, which wraps a future in a `Budget`: a future that runs the wrapped future, but
gives up if it has not finished after a given number of polls. There is no async runtime (only `std`
is available). The skeleton below is already in the editor; choose the fields of `Budget` yourself.

```rust
pub struct Budget<F> { /* private */ }

pub fn with_budget<F: Future>(fut: F, polls: u32) -> Budget<F>;

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;
    /* poll */
}
```

Below, `fut` is called the *inner future*.

- Calling `with_budget` polls nothing and drops nothing.
- Each poll of the `Budget` polls the inner future once, passing on the `Context` that the `Budget`
  is being polled with. Then:
  - If the inner future returned `Poll::Ready(v)`, the `Budget` returns `Poll::Ready(Some(v))`.
  - If it returned `Poll::Pending`, the `Budget` returns `Poll::Pending`, unless that was the
    `polls`-th poll of the inner future. Then the budget is used up, and the `Budget` gives up right
    away: it returns `Poll::Ready(None)` in that same poll.
- If `polls` is `0`, the first poll of the `Budget` returns `Poll::Ready(None)` without polling the
  inner future.
- The inner future is dropped as soon as it is no longer needed. Whenever the `Budget` returns
  `Poll::Ready`, it has already dropped the inner future during that poll, before returning; it does
  not wait until the `Budget` itself is dropped. A `Budget` that is dropped before it is ready drops
  the inner future with it.
- The `Budget` never wakes the waker itself; waking it is up to the inner future.
- The inner future can be any future, including an `async` block that borrows local variables and
  is neither `Unpin` nor `Send`. Its output can be of any type.

The test harness polls the futures on the current thread. It never polls a `Budget` again after it
has returned `Poll::Ready`.

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
