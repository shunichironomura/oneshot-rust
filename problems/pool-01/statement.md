A thread pool keeps a fixed number of worker threads alive and hands them jobs, instead of
starting a new thread for every job. Write one that also shuts down cleanly. The skeleton below is
already in the editor. Choose the fields of `ThreadPool` yourself, and add any types and functions
you need.

```rust
pub struct ThreadPool { /* private */ }

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool;
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static;
    pub fn shutdown(self) -> usize;
}

impl Drop for ThreadPool { /* ... */ }
```

- `new(size)` starts `size` worker threads and returns the pool. It panics if `size` is `0`.
- `execute(f)` adds the job `f` to the pool's queue and returns at once, without waiting for the
  job to start.
- Every job runs exactly once, on one of the pool's worker threads: never on the thread that
  called `execute`, and never on a thread started just for that job.
- A worker runs one job at a time. When it is free, it takes the job that has been waiting the
  longest (first in, first out). A waiting job is taken by any free worker, even while the other
  workers are busy, so a pool of size `n` runs up to `n` jobs at the same time. A pool of size 1
  runs its jobs one after another, in the order they were added.
- Several threads may call `execute` on the same pool at the same time: the harness shares a
  `&ThreadPool` between threads with `std::thread::scope`.
- `shutdown()` waits until every job added to the pool has finished, including the jobs that are
  still waiting in the queue. Then it stops the worker threads, waits for them to end, and returns
  the number of jobs the pool has run. Every job runs, so this is the number of `execute` calls.
- Dropping the pool without calling `shutdown` does the same, except that there is no count to
  return: when the drop is over, every job has finished and every worker thread has ended.
- Jobs never panic. A job may wait for another job of the same pool (for example at a `Barrier`),
  but the harness never makes a thread wait for something that cannot happen. (When every thread
  waits forever, Miri reports a deadlock.)

Do not change the signatures.

### Examples

```rust
let pool = ThreadPool::new(2);
let total = Arc::new(AtomicUsize::new(0));
for i in 1..=3 {
    let total = Arc::clone(&total);
    pool.execute(move || {
        total.fetch_add(i, SeqCst);
    });
}
pool.shutdown()       // 3: the pool has run 3 jobs
total.load(SeqCst)    // 6 = 1 + 2 + 3
```

A pool of size 1 gets three jobs that push `"a"`, `"b"` and `"c"` to a shared `Vec`, and is then
dropped. Once the drop is over, the `Vec` is `["a", "b", "c"]`.
