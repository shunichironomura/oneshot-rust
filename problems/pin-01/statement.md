The code below does not compile. Fix it.

Each function runs a few tasks, written as `async` blocks, on a tiny round-robin executor (only
`std` is available; there is no async runtime).

```rust
pub fn squares_round_robin(n: u32) -> Vec<u32>;
pub fn finish_order(delays: &[u32]) -> Vec<usize>;
```

- A task yields by awaiting `yield_now()`, which returns `Poll::Pending` the first time it is
  polled (after waking the task) and `Poll::Ready(())` the next time. It is correct as it is.
- The executor, `run_round_robin`, works in rounds. In each round, it polls every task that has not
  finished yet once, in index order. It repeats rounds until all tasks have finished, and it never
  polls a finished task again. A task that yields `k` times therefore finishes in round `k + 1`.
- `squares_round_robin(n)` runs `n` tasks, with indices `0` to `n - 1`. Task `i` computes `i * i`
  by adding `i` to a total `i` times, yielding after each addition. The result holds the output of
  each task, in index order. It is empty for `n = 0`.
- `finish_order(delays)` runs one task per element of `delays`: task `i` yields `delays[i]` times
  and then finishes. The result lists the indices of the tasks in the order they finished. Tasks
  that finish in the same round are listed in index order. It is empty when `delays` is empty.

Do not change the public signatures. Keep the tasks as `async` blocks that are run by the
round-robin executor.

### Examples

- `squares_round_robin(4)` returns `[0, 1, 4, 9]`.
- `finish_order(&[2, 0, 1])` returns `[1, 2, 0]`: task 1 finishes in round 1, task 2 in round 2
  and task 0 in round 3.
