The model solution follows the thread pool of chapter 21 of the Book. The pool keeps the sending
half of one `mpsc` channel, and each worker loops: receive the next job, run it. A `Receiver`
cannot be cloned and is not `Sync`, so the workers share it as `Arc<Mutex<Receiver<Job>>>`, and the
mutex makes sure that each job is received by exactly one worker.

Every closure has its own type, so the queue holds the jobs as trait objects:
`type Job = Box<dyn FnOnce() + Send + 'static>`. `FnOnce` because each job runs once (and may
consume what it captured), `Send` because it moves to a worker thread, and `'static` because the
worker may run it long after `execute` has returned. A `Box<dyn FnOnce()>` can be called directly
with `job()`.

**Stopping in the right order.** `recv` keeps handing out the jobs in the queue and fails only
when the queue is empty *and* every `Sender` is gone. So `stop` first drops the pool's `Sender`,
which lets the workers finish the queue and leave their loops, and only then joins them. If you
join first, the workers wait in `recv` forever, `join` waits for them, and Miri reports a
deadlock. The sender is an `Option<Sender<Job>>` so that `stop` can drop it through `&mut self`
with `take()`.

**Moving out of `self`.** `JoinHandle::join` takes the handle by value, so
`for w in &mut self.workers { w.join() }` does not compile (E0507). `self.workers.drain(..)` moves
the handles out and leaves an empty `Vec` behind. `shutdown(self)` has a similar problem: since
`ThreadPool` implements `Drop`, you cannot move a field out of it or destructure it (E0509),
because `drop` will still run on the whole value. And `drop` only gets `&mut self`, so it cannot
call `shutdown`. The way out is one helper, `stop(&mut self)`, used by both. It leaves the pool
stopped and empty: when `shutdown` returns, `self` is dropped, and the second `stop` finds no
sender and no workers, so it does nothing. (`mut self` in `shutdown` does not change its
signature.)

**Releasing the lock before running the job.** In `let message = receiver.lock().unwrap().recv();`
the `MutexGuard` is a temporary that is dropped at the end of the `let` statement, before the job
runs. The shorter `while let Ok(job) = receiver.lock().unwrap().recv() { job(); }` behaves
differently: the temporaries of a `while let`, `if let` or `match` scrutinee live until the end of
its block, so the worker holds the lock while the job runs. The other workers cannot even take a
job, and the pool runs one job at a time. When jobs wait for each other, as in the hidden cases,
that is a deadlock. (Since edition 2024, the temporaries of an `if let` are dropped before its
`else` block, but they are still alive in the block that runs when the pattern matches.)

**Counting.** Each worker counts the jobs it ran and returns the number from its thread, so the
handles are `JoinHandle<usize>` and `stop` adds up what `join` returns. Counting in `execute` works
too, but `execute` takes `&self` and the pool is shared between threads: a `Cell<usize>` makes
`ThreadPool` not `Sync`, and the harness does not compile. Use an `AtomicUsize` instead (first
alternative). `Sender` is `Sync` (since Rust 1.72), so the model solution needs no mutex around
it. If the workers add to a shared counter, read it only after joining them, or the jobs still
in the queue are missing from the count.

### Alternatives

Count in `execute` with an `AtomicUsize`, and use `std::mem::take` instead of `drain`. `shutdown`
owns `self`, so it reads the counter with `get_mut`, without an atomic operation. The guard of a
`let ... else` is also dropped at the end of the statement.

```rust
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Option<Sender<Job>>,
    added: AtomicUsize,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(thread::spawn(move || {
                loop {
                    // The guard is dropped at the end of this `let ... else` statement.
                    let Ok(job) = receiver.lock().unwrap().recv() else {
                        break;
                    };
                    job();
                }
            }));
        }
        ThreadPool { workers, sender: Some(sender), added: AtomicUsize::new(0) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.added.fetch_add(1, Ordering::Relaxed);
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }

    pub fn shutdown(mut self) -> usize {
        self.stop();
        *self.added.get_mut()
    }

    fn stop(&mut self) {
        drop(self.sender.take());
        for worker in mem::take(&mut self.workers) {
            worker.join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.stop();
    }
}
```

Keep `while let`, but lock inside a function that returns an `Option<Job>`. The guard is dropped
when the function returns, so nothing is locked while the job runs.

```rust
fn run_worker(receiver: &Mutex<Receiver<Job>>) -> usize {
    let mut ran = 0;
    while let Some(job) = next_job(receiver) {
        job();
        ran += 1;
    }
    ran
}

/// Waits for the next job. The guard is dropped when this function returns.
fn next_job(receiver: &Mutex<Receiver<Job>>) -> Option<Job> {
    receiver.lock().unwrap().recv().ok()
}
```

The `Option` around the sender can be avoided by swapping in a `Sender` of a channel that is
already closed (its `Receiver` is dropped right away). The field is then a plain `Sender<Job>`,
and `execute` calls `self.sender.send(...)` directly.

```rust
fn stop(&mut self) -> usize {
    let (closed, _) = mpsc::channel();
    drop(mem::replace(&mut self.sender, closed));
    self.workers.drain(..).map(|worker| worker.join().unwrap()).sum()
}
```

Without a channel: a `Mutex<VecDeque<Job>>` with a closed flag, and a `Condvar` that the workers
sleep on while the queue is empty and open. Closing must wake every worker, hence `notify_all`.

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Queue {
    jobs: VecDeque<Job>,
    closed: bool,
}

struct Shared {
    queue: Mutex<Queue>,
    /// Notified when a job is added or the queue is closed.
    changed: Condvar,
}

impl Shared {
    /// Waits for the next job. Returns `None` once the queue is closed and empty.
    fn next_job(&self) -> Option<Job> {
        let queue = self.queue.lock().unwrap();
        let mut queue = self
            .changed
            .wait_while(queue, |q| q.jobs.is_empty() && !q.closed)
            .unwrap();
        queue.jobs.pop_front()
    }
}

pub struct ThreadPool {
    shared: Arc<Shared>,
    workers: Vec<JoinHandle<usize>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let shared = Arc::new(Shared {
            queue: Mutex::new(Queue { jobs: VecDeque::new(), closed: false }),
            changed: Condvar::new(),
        });
        let workers = (0..size)
            .map(|_| {
                let shared = Arc::clone(&shared);
                thread::spawn(move || {
                    let mut ran = 0;
                    // `next_job` releases the lock before it returns.
                    while let Some(job) = shared.next_job() {
                        job();
                        ran += 1;
                    }
                    ran
                })
            })
            .collect();
        ThreadPool { shared, workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.shared.queue.lock().unwrap().jobs.push_back(Box::new(f));
        self.shared.changed.notify_one();
    }

    pub fn shutdown(mut self) -> usize {
        self.stop()
    }

    fn stop(&mut self) -> usize {
        self.shared.queue.lock().unwrap().closed = true;
        self.shared.changed.notify_all();
        self.workers.drain(..).map(|worker| worker.join().unwrap()).sum()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.stop();
    }
}
```
