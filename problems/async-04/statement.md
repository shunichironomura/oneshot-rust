The code below does not compile. Fix it.

Async tasks share a log, a `Vec<String>` behind a `std::sync::Mutex`.

```rust
pub async fn yield_now();
pub async fn record_all(log: &Mutex<Vec<String>>, items: Vec<String>) -> usize;
pub async fn forward_all(inbox: &Mutex<Vec<String>>, log: &Mutex<Vec<String>>) -> usize;
```

- The future of `yield_now()` returns `Poll::Pending` the first time it is polled (after waking the
  task) and `Poll::Ready(())` the next time. It is correct as it is.
- `record_all(log, items)` handles the items one at a time, in order. For each item, it locks
  `log`, pushes the item, notes the new length of `log`, unlocks `log`, and then awaits
  `yield_now()`. It resolves to the length noted after the last push. When `items` is empty, it
  resolves to the current length of `log`, without yielding.
- `forward_all(inbox, log)` locks `inbox`, takes all its messages at once (leaving it empty) and
  unlocks it. Then it records the messages in `log`, in their order, with `record_all`, and resolves
  to what `record_all` resolves to. `inbox` and `log` are different mutexes.

The test harness runs the futures with its own small executor (only `std` is available). Some tests
create futures on the main thread and run them to completion on another thread, so the futures must
be `Send`. Some tests poll two futures in turns (first, second, first, ...) until both are ready.
While a future is waiting, other futures and the harness itself may lock `log` and `inbox` and push
to them.

Do not change the signatures or `yield_now`. Keep `std::sync::Mutex` and the yields.

### Examples

- With an empty `log`, `record_all(&log, vec!["a".into(), "b".into()])` resolves to `2` and leaves
  `log` as `["a", "b"]`. It returns `Poll::Pending` twice, once after each push.
- With `inbox` = `["x", "y"]` and `log` = `["start"]`, `forward_all(&inbox, &log)` resolves to `3`,
  leaves `log` as `["start", "x", "y"]` and empties `inbox`.
