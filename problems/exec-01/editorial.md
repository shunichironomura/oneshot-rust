A `Waker` can be made from an `Arc<W>` where `W` implements `std::task::Wake`, but
`From<Arc<W>> for Waker` requires `W: Wake + Send + Sync + 'static`: a waker may be cloned and woken
on any thread. The tasks here are not `Send` (they hold `Rc`s), so the classic design where the
waker is an `Arc<Task>` that owns the future in a `Mutex` does not compile:
"`dyn Future<Output = ()>` cannot be sent between threads safely" at `Waker::from`. Adding `+ Send`
to the boxed future only moves the error to `spawn`, and an `Rc<RefCell<VecDeque<_>>>` queue fails
for the same reason. So the task is split in two:

- The executor owns the futures, as `Pin<Box<dyn Future<Output = ()>>>` indexed by task id. Polling
  needs a pinned future; `Box::pin` pins any future, and `as_mut()` reborrows the pin for each poll.
- The waker knows only the id of its task and the ready queue, an `Arc<Mutex<VecDeque<usize>>>`.
  These are `Send + Sync`, so `Waker::from(Arc::new(TaskWaker { .. }))` compiles.

Waking pushes the id onto the queue, and `run` pops ids and polls the matching futures. The rest is
bookkeeping:

- **Deduplication.** Each task has a `queued` flag, an `AtomicBool` because the waker must be
  `Sync`. Only the wake that turns it from `false` to `true` pushes the id, so the task keeps the
  place of its first wake. `run` clears the flag *before* the poll. Cleared after it, a wake during
  the poll (as in task A of the example) would find `true`, push nothing, and the task would never
  run again. Pushing every wake and checking the flag when popping is not the same either: a stale
  entry from an earlier wake can then let a task overtake tasks that became ready before it.
- **Finished tasks.** The queue may still contain the id of a finished task (it woke itself and then
  returned `Ready`), and a stored waker may be woken at any time. The slot of a finished task
  becomes `None` (which drops the future), and `run` skips such ids. Ids are never reused, so an old
  waker cannot wake a newer task. With `id = tasks.len()` on a `HashMap` that removes finished
  tasks, it could.
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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::task::{Context, Wake, Waker};

struct TaskWaker {
    id: usize,
    queued: AtomicBool,
    sender: Sender<usize>,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        if !self.queued.swap(true, Ordering::SeqCst) {
            // Fails only when the executor is gone; then there is nothing left to run.
            let _ = self.sender.send(self.id);
        }
    }
}

type Task = (Pin<Box<dyn Future<Output = ()>>>, Arc<TaskWaker>);

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
        let waker = Arc::new(TaskWaker {
            id: self.tasks.len(),
            queued: AtomicBool::new(false),
            sender: self.sender.clone(),
        });
        waker.wake_by_ref();
        self.tasks.push(Some((Box::pin(fut), waker)));
    }

    pub fn run(&mut self) -> usize {
        let mut polls = 0;
        while let Ok(id) = self.receiver.try_recv() {
            let slot = &mut self.tasks[id];
            let Some((future, task_waker)) = slot else {
                continue;
            };
            task_waker.queued.store(false, Ordering::SeqCst);
            let waker = Waker::from(Arc::clone(task_waker));
            polls += 1;
            if future.as_mut().poll(&mut Context::from_waker(&waker)).is_ready() {
                *slot = None;
            }
        }
        polls
    }

    pub fn pending(&self) -> usize {
        self.tasks.iter().filter(|task| task.is_some()).count()
    }
}
```

The flags can also live next to the queue, behind the same `Mutex`, so that no atomics are needed.
This version keeps the tasks in a `HashMap` and makes the `Waker` of each task only once.

```rust
use std::collections::{HashMap, VecDeque};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Wake, Waker};

/// Everything the wakers change, behind one lock.
#[derive(Default)]
struct ReadyQueue {
    order: VecDeque<usize>,
    /// `queued[id]`: whether task `id` is in `order`.
    queued: Vec<bool>,
}

impl ReadyQueue {
    /// Registers a new task, which is ready, and returns its id.
    fn add(&mut self) -> usize {
        let id = self.queued.len();
        self.queued.push(true);
        self.order.push_back(id);
        id
    }

    fn wake(&mut self, id: usize) {
        if !self.queued[id] {
            self.queued[id] = true;
            self.order.push_back(id);
        }
    }

    fn pop(&mut self) -> Option<usize> {
        let id = self.order.pop_front()?;
        self.queued[id] = false;
        Some(id)
    }
}

struct TaskWaker {
    id: usize,
    ready: Arc<Mutex<ReadyQueue>>,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.ready.lock().unwrap().wake(self.id);
    }
}

pub struct Executor {
    tasks: HashMap<usize, (Pin<Box<dyn Future<Output = ()>>>, Waker)>,
    ready: Arc<Mutex<ReadyQueue>>,
}

impl Executor {
    pub fn new() -> Self {
        Executor { tasks: HashMap::new(), ready: Arc::default() }
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        let id = self.ready.lock().unwrap().add();
        let waker = Waker::from(Arc::new(TaskWaker { id, ready: Arc::clone(&self.ready) }));
        self.tasks.insert(id, (Box::pin(fut), waker));
    }

    pub fn run(&mut self) -> usize {
        let mut polls = 0;
        loop {
            let next = self.ready.lock().unwrap().pop();
            let Some(id) = next else {
                return polls;
            };
            let Some((future, waker)) = self.tasks.get_mut(&id) else {
                continue;
            };
            polls += 1;
            if future.as_mut().poll(&mut Context::from_waker(waker)).is_ready() {
                self.tasks.remove(&id);
            }
        }
    }

    pub fn pending(&self) -> usize {
        self.tasks.len()
    }
}
```
