Write `join` and `join_all`, which combine several futures into one future that runs them
concurrently by polling them in turns on the current thread. There is no async runtime (only `std`
is available).

```rust
pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)>;
pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>>;
```

Below, the future returned by `join` or `join_all` is called the *joined future*, and `a` and `b`,
or the elements of `futs`, are its *inputs*.

- Calling `join` or `join_all` polls nothing. The inputs are polled only when the joined future is
  polled.
- Each time the joined future is polled, it polls each input that has not completed yet once, in
  order: `a` then `b`, and the elements of `futs` in index order. An input that returns
  `Poll::Pending` does not stop the inputs after it from being polled.
- An input has completed once it has returned `Poll::Ready`. It is never polled again.
- The joined future returns `Poll::Ready` in the same poll in which its last input completes, and
  `Poll::Pending` in every poll before that. The outputs are in input order: `(output of a, output
  of b)`, or a `Vec` holding the output of each element of `futs`. When `futs` is empty, the joined
  future is ready with an empty `Vec` on its first poll.
- The inputs are polled with the `Context` that the joined future is being polled with. The joined
  future never wakes the waker itself; waking it is up to the inputs.
- The inputs can be any futures, including `async` blocks that borrow local variables and are
  neither `Unpin` nor `Send`. Their outputs can be of any type, even one that implements none of
  `Clone`, `Copy`, `Default` and `Unpin`.

The test harness runs the joined futures on the current thread with an executor of its own. Like a
real executor, it polls a future again only after the waker has been woken: a future that returns
`Poll::Pending` without that happening would never be polled again, and the harness reports it as
a failure. It never polls a future again after it has returned `Poll::Ready`.

### Examples

In the examples, `steps(["a0", "a1"])` stands for a task of the harness: an `async` block that logs
the given steps one at a time. Between two steps, it yields once: it wakes the waker and returns
`Poll::Pending`. So it completes in the poll that logs its last step (in its first poll if it has
no steps), and its output is the number of steps.

- `join(steps(["a0", "a1"]), steps(["b0", "b1", "b2"]))` logs `a0`, `b0`, `a1`, `b1`, `b2` and is
  ready with `(2, 3)` on its third poll. The first input completes in the second poll, so the third
  poll polls only the second input.
- `join_all(vec![steps(["x0", "x1", "x2"]), steps(["y0"]), steps(["z0", "z1"])])` logs `x0`, `y0`,
  `z0`, `x1`, `z1`, `x2` and is ready with `[3, 1, 2]` on its third poll.
