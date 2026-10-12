use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Wake, Waker};

/// The ids of the ready tasks, in the order they became ready.
type ReadyQueue = Arc<Mutex<VecDeque<usize>>>;

/// What the wakers of one task share. `Waker::from` needs it to be `Send + Sync`,
/// so it knows the task only by its id; the future itself stays in the executor.
struct TaskWaker {
    id: usize,
    ready: ReadyQueue,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.ready.lock().unwrap().push_back(self.id);
    }
}

/// A spawned task: its future, and the waker to poll it with.
struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    waker: Waker,
}

pub struct Executor {
    /// Indexed by task id, `None` once the task has finished. Ids are never reused,
    /// so a waker of a finished task cannot wake a newer task.
    tasks: Vec<Option<Task>>,
    ready: ReadyQueue,
}

impl Executor {
    pub fn new() -> Self {
        Executor { tasks: Vec::new(), ready: ReadyQueue::default() }
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        let id = self.tasks.len();
        let waker = Waker::from(Arc::new(TaskWaker { id, ready: Arc::clone(&self.ready) }));
        waker.wake_by_ref();
        self.tasks.push(Some(Task { future: Box::pin(fut), waker }));
    }

    pub fn run(&mut self) -> usize {
        let mut polls = 0;
        loop {
            // The lock guard is a temporary of this statement, so the lock is released
            // before the poll, during which the task may wake itself.
            let Some(id) = self.ready.lock().unwrap().pop_front() else {
                return polls;
            };
            let slot = &mut self.tasks[id];
            // A task can finish after it was queued; it stays finished.
            let Some(task) = slot else {
                continue;
            };
            polls += 1;
            let mut cx = Context::from_waker(&task.waker);
            if task.future.as_mut().poll(&mut cx).is_ready() {
                *slot = None;
            }
        }
    }
}
