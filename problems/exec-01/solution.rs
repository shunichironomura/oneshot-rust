use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Wake, Waker};

/// The ids of the ready tasks, in the order they became ready.
type ReadyQueue = Arc<Mutex<VecDeque<usize>>>;

/// What the wakers of one task share. `Waker::from` needs it to be `Send + Sync`,
/// so it knows the task only by its id; the future itself stays in the executor.
struct TaskWaker {
    id: usize,
    /// Whether the task is ready, that is, in the queue.
    queued: AtomicBool,
    ready: ReadyQueue,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        // Only the wake that makes the task ready puts it in the queue.
        if !self.queued.swap(true, Ordering::SeqCst) {
            self.ready.lock().unwrap().push_back(self.id);
        }
    }
}

struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    waker: Arc<TaskWaker>,
}

pub struct Executor {
    /// Indexed by task id, `None` once the task has finished. Ids are never reused,
    /// so a waker of a finished task cannot wake a newer task.
    tasks: Vec<Option<Task>>,
    pending: usize,
    ready: ReadyQueue,
}

impl Executor {
    pub fn new() -> Self {
        Executor { tasks: Vec::new(), pending: 0, ready: ReadyQueue::default() }
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        let waker = Arc::new(TaskWaker {
            id: self.tasks.len(),
            queued: AtomicBool::new(false),
            ready: Arc::clone(&self.ready),
        });
        waker.wake_by_ref();
        self.tasks.push(Some(Task { future: Box::pin(fut), waker }));
        self.pending += 1;
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
            // Not ready any more: a wake from now on, even during this poll, queues it again.
            task.waker.queued.store(false, Ordering::SeqCst);
            let waker = Waker::from(Arc::clone(&task.waker));
            let mut cx = Context::from_waker(&waker);
            polls += 1;
            if task.future.as_mut().poll(&mut cx).is_ready() {
                *slot = None;
                self.pending -= 1;
            }
        }
    }

    pub fn pending(&self) -> usize {
        self.pending
    }
}
