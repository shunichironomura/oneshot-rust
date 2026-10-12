use std::panic;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::{Arc, Barrier, Mutex};
use std::thread::{self, ThreadId};

use crate::judge::{case, check, check_eq};
use crate::user::ThreadPool;

/// What the jobs did: a label and the thread each job ran on.
type Log<T> = Arc<Mutex<Vec<(T, ThreadId)>>>;

/// Gives the other threads some turns.
fn let_others_run() {
    for _ in 0..3 {
        thread::yield_now();
    }
}

/// A job that appends `x` and its thread to `log`, after a while.
fn log_later<T: Send + 'static>(log: &Log<T>, x: T) -> impl FnOnce() + Send + 'static {
    let log = Arc::clone(log);
    move || {
        let_others_run();
        log.lock().unwrap().push((x, thread::current().id()));
    }
}

/// A job that waits at `barrier`, then appends `x` and its thread to `log`.
fn meet<T: Send + 'static>(
    barrier: &Arc<Barrier>,
    log: &Log<T>,
    x: T,
) -> impl FnOnce() + Send + 'static {
    let barrier = Arc::clone(barrier);
    let log = Arc::clone(log);
    move || {
        barrier.wait();
        log.lock().unwrap().push((x, thread::current().id()));
    }
}

pub fn run() {
    case("sample-1", || {
        let pool = ThreadPool::new(2);
        let total = Arc::new(AtomicUsize::new(0));
        for i in 1..=3 {
            let total = Arc::clone(&total);
            pool.execute(move || {
                total.fetch_add(i, SeqCst);
            });
        }
        pool.shutdown();
        check_eq(total.load(SeqCst), 6)
    });
    case("sample-2", || {
        let pool = ThreadPool::new(1);
        let log = Arc::new(Mutex::new(Vec::new()));
        for word in ["a", "b", "c"] {
            let log = Arc::clone(&log);
            pool.execute(move || log.lock().unwrap().push(word));
        }
        drop(pool);
        check_eq(log.lock().unwrap().clone(), vec!["a", "b", "c"])
    });
    case("hidden-1", || {
        // new(0) panics, and a pool without jobs shuts down.
        check(panic::catch_unwind(|| ThreadPool::new(0)).is_err(), "new(0) did not panic")?;
        ThreadPool::new(1).shutdown();
        Ok(())
    });
    case("hidden-2", || {
        // execute returns before the job runs, even while the only worker is busy: the first job
        // waits until the main thread meets it at the barrier, and three more jobs are added
        // meanwhile. Dropping the pool waits for the jobs still in its queue. A pool of size 1
        // runs its jobs in order, all on the same worker thread.
        let pool = ThreadPool::new(1);
        let barrier = Arc::new(Barrier::new(2));
        let log = Log::default();
        pool.execute(meet(&barrier, &log, 0));
        for i in 1..=3 {
            pool.execute(log_later(&log, i));
        }
        barrier.wait();
        drop(pool);
        let log = log.lock().unwrap().clone();
        let order: Vec<i32> = log.iter().map(|&(i, _)| i).collect();
        check_eq(order, vec![0, 1, 2, 3])?;
        let worker = log[0].1;
        check(worker != thread::current().id(), "a job ran on the thread that called execute")?;
        check(log.iter().all(|&(_, id)| id == worker), "a pool of size 1 ran jobs on two threads")
    });
    case("hidden-3", || {
        // Two jobs wait at a barrier until both have arrived, so a pool of 2 must run them at the
        // same time. A short job comes between them: the free worker takes it, then the second
        // waiting job, while the other worker is busy. The last job is added by another thread.
        let pool = ThreadPool::new(2);
        let barrier = Arc::new(Barrier::new(2));
        let log = Log::default();
        pool.execute(meet(&barrier, &log, "first"));
        pool.execute(log_later(&log, "short"));
        thread::scope(|s| {
            s.spawn(|| pool.execute(meet(&barrier, &log, "last")));
        });
        pool.shutdown();
        let log = log.lock().unwrap().clone();
        check_eq(log.len(), 3)?;
        check_eq(log[0].0, "short")?;
        let main = thread::current().id();
        check(log.iter().all(|&(_, id)| id != main), "a job ran on the main thread")?;
        check(log[1].1 != log[2].1, "two jobs that met at a barrier ran on the same thread")
    });
}
