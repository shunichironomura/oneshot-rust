A `Waker` can be made from an `Arc<W>` where `W` implements `std::task::Wake`, but
`From<Arc<W>> for Waker` requires `W: Wake + Send + Sync + 'static`: a waker may be cloned and woken
on any thread. The tasks here are not `Send` (they hold `Rc`s), so the classic design where the
waker is an `Arc<Task>` that owns the future in a `Mutex` does not compile: at `Waker::from`, rustc
reports that `dyn Future<Output = ()>` cannot be sent between threads safely. Adding `+ Send` to the
boxed future only moves the error to `spawn`, and an `Rc<RefCell<VecDeque<_>>>` queue fails for the
same reason. So the task is split in two:

- The executor owns the futures, as `Pin<Box<dyn Future<Output = ()>>>` indexed by task id. Polling
  needs a pinned future; `Box::pin` pins any future, and `as_mut()` reborrows the pin for each poll.
- The waker knows only the id of its task and the ready queue, an `Arc<Mutex<VecDeque<usize>>>`.
  These are `Send + Sync`, so `Waker::from(Arc::new(TaskWaker { .. }))` compiles. `spawn` makes the
  `Waker` once and keeps it in the `Task`, next to the future.

Waking pushes the id onto the queue, and `run` pops ids and polls the matching futures. The harness
never wakes a task that is already ready, so a wake can always push. (A real executor also has to
handle that case, usually with a "queued" flag per task, so that a task woken twice is polled once.)
The rest is bookkeeping:

- **Finished tasks.** The queue may still contain the id of a finished task (it woke itself and then
  returned `Ready`), and a stored waker may be woken at any time. The slot of a finished task
  becomes `None` (which drops the future), and `run` skips such ids with `continue` (not `break` or
  `return`: other tasks may be ready behind them). Ids are never reused, so an old waker cannot wake
  a newer task. With `id = tasks.len()` on a `HashMap` that removes finished tasks, it could.
- **Do not hold the lock while polling.** In
  `while let Some(id) = self.ready.lock().unwrap().pop_front() { ... }`, the `MutexGuard` is a
  temporary of the condition and lives until the end of the loop body (the same goes for the
  scrutinee of a `match`, and for `if let` up to the end of its first block). A task that wakes
  itself during its poll then locks the queue a second time on the same thread, and Miri reports a
  deadlock. With `let ... else`, temporaries are dropped at the end of the statement, so the guard
  is gone before the poll.

### Alternatives

`mpsc::Sender` is `Send + Sync` (since Rust 1.72), so the waker can hold a sender of the task id,
and `run` drains the receiver with `try_recv`. Sending fails once the executor (with its receiver)
has been dropped. Ignore that error instead of unwrapping it.

```rust
use std::pin::Pin;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::task::{Context, Wake, Waker};

/// A spawned task: its future, and the waker to poll it with.
struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    waker: Waker,
}

struct TaskWaker {
    id: usize,
    sender: Sender<usize>,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        // Fails only when the executor is gone; then there is nothing left to run.
        let _ = self.sender.send(self.id);
    }
}

pub struct Executor {
    tasks: Vec<Option<Task>>,
    sender: Sender<usize>,
    receiver: Receiver<usize>,
}

impl Executor {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Executor { tasks: Vec::new(), sender, receiver }
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        let id = self.tasks.len();
        let waker = Waker::from(Arc::new(TaskWaker { id, sender: self.sender.clone() }));
        waker.wake_by_ref();
        self.tasks.push(Some(Task { future: Box::pin(fut), waker }));
    }

    pub fn run(&mut self) -> usize {
        let mut polls = 0;
        while let Ok(id) = self.receiver.try_recv() {
            let slot = &mut self.tasks[id];
            let Some(task) = slot else {
                continue;
            };
            polls += 1;
            if task.future.as_mut().poll(&mut Context::from_waker(&task.waker)).is_ready() {
                *slot = None;
            }
        }
        polls
    }
}
```
