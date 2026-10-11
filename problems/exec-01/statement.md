Write a small executor that runs many tasks on the current thread and uses wakers to decide which
task to poll next. The skeleton below is already in the editor. Choose the fields of `Executor`
yourself, and add any types you need.

```rust
pub struct Executor { /* private */ }

impl Executor {
    pub fn new() -> Self;
    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static);
    pub fn run(&mut self) -> usize;
    pub fn pending(&self) -> usize;
}
```

- `new` creates an executor without tasks. `spawn` adds a task that runs `fut`. It does not poll the
  task, but a newly spawned task is *ready*.
- `run` repeatedly takes the task that has been ready the longest (first in, first out) and polls it
  once, until no task is ready. It returns how many polls it made (`0` if no task was ready).
- A task stops being ready when `run` takes it. It becomes ready again only when one of its wakers
  is woken with `wake` or `wake_by_ref`. The wakers of a task are the waker of the `Context` passed
  to any of its polls, and the clones of that waker. A waker may be woken during any poll (including
  a poll of its own task), after `run` has returned, and on any thread.
- Waking a task that is already ready does nothing. A task woken several times before its next poll
  is polled once, and its place in the order is that of the first wake.
- A task whose poll returns `Poll::Ready` has finished: its future is dropped right away, and it is
  never polled again. Waking a finished task does nothing.
- `pending` returns the number of spawned tasks that have not finished. Those tasks are dropped
  together with the executor.
- The futures do not have to be `Send`: the tasks of the harness hold `Rc<RefCell<...>>` values. A
  waker may outlive its executor; waking it then does nothing (in particular, it does not panic).

### Examples

- Task A logs `"a1"`, wakes itself with `wake_by_ref` and returns `Pending`; on its next poll it
  logs `"a2"` and finishes. Task B logs `"b1"` and finishes. After spawning A and then B, `run()`
  returns `3`, the log is `["a1", "b1", "a2"]`, and `pending()` is `0`.
- Task C stores a clone of its waker in a shared slot and returns `Pending` without waking anything;
  on its next poll it finishes. After spawning C, `run()` returns `1` and `pending()` is `1`.
  Nothing has woken C, so calling `run()` again returns `0`. After the waker in the slot is woken,
  `run()` returns `1` and `pending()` is `0`.
