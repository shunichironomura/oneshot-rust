use std::panic;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::{Arc, Barrier, Mutex};
use std::thread::{self, ThreadId};

use crate::judge::{case, check, check_eq};
use crate::user::ThreadPool;

/// Gives the other threads some turns.
fn let_others_run() {
    for _ in 0..10 {
        thread::yield_now();
    }
}

/// A job that appends `x` to `log`, after a while.
fn log_later<T: Send + 'static>(log: &Arc<Mutex<Vec<T>>>, x: T) -> impl FnOnce() + Send + 'static {
    let log = Arc::clone(log);
    move || {
        let_others_run();
        log.lock().unwrap().push(x);
    }
}

/// A job that waits at `barrier`, then appends the id of its thread to `ids`.
fn meet(barrier: &Arc<Barrier>, ids: &Arc<Mutex<Vec<ThreadId>>>) -> impl FnOnce() + Send + 'static {
    let barrier = Arc::clone(barrier);
    let ids = Arc::clone(ids);
    move || {
        barrier.wait();
        ids.lock().unwrap().push(thread::current().id());
    }
}

fn take<T: Clone>(log: &Arc<Mutex<Vec<T>>>) -> Vec<T> {
    log.lock().unwrap().clone()
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
        check_eq(pool.shutdown(), 3)?;
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
        check_eq(take(&log), vec!["a", "b", "c"])
    });
    case("hidden-1", || {
        // new(0) panics, and a pool without jobs has run 0 jobs.
        check(panic::catch_unwind(|| ThreadPool::new(0)).is_err(), "new(0) did not panic")?;
        check_eq(ThreadPool::new(1).shutdown(), 0)
    });
    case("hidden-2", || {
        // Dropping a pool waits for the jobs that are still in its queue. Each pool counts its
        // own jobs. execute returns before the job runs: the first job of `a` waits until the
        // main thread meets it at the barrier. A pool of size 1 runs every job on the same worker
        // thread, which stays alive between jobs.
        let a = ThreadPool::new(1);
        let b = ThreadPool::new(1);
        let log = Arc::new(Mutex::new(Vec::new()));
        for i in 0..3 {
            b.execute(log_later(&log, i));
        }
        drop(b);
        check_eq(take(&log), vec![0, 1, 2])?;
        let barrier = Arc::new(Barrier::new(2));
        let ids = Arc::new(Mutex::new(Vec::new()));
        a.execute(meet(&barrier, &ids));
        barrier.wait();
        let_others_run();
        let worker_ids = Arc::clone(&ids);
        a.execute(move || worker_ids.lock().unwrap().push(thread::current().id()));
        check_eq(a.shutdown(), 2)?;
        let ids = take(&ids);
        check_eq(ids.len(), 2)?;
        check(ids[0] != thread::current().id(), "a job ran on the thread that called execute")?;
        check(ids[0] == ids[1], "a pool of size 1 ran jobs on two threads")
    });
    case("hidden-3", || {
        // Two jobs wait at a barrier until both have arrived, so a pool of 2 must run them at the
        // same time. A short job comes between them: the free worker takes it, then the second
        // waiting job, while the other worker is busy. The last job is added by another thread.
        let pool = ThreadPool::new(2);
        let barrier = Arc::new(Barrier::new(2));
        let ids = Arc::new(Mutex::new(Vec::new()));
        let log = Arc::new(Mutex::new(Vec::new()));
        let first = meet(&barrier, &ids);
        let after_meeting = log_later(&log, "first");
        pool.execute(move || {
            first();
            after_meeting();
        });
        pool.execute(log_later(&log, "short"));
        thread::scope(|s| {
            s.spawn(|| pool.execute(meet(&barrier, &ids)));
        });
        check_eq(pool.shutdown(), 3)?;
        check_eq(take(&log), vec!["short", "first"])?;
        let ids = take(&ids);
        check(!ids.contains(&thread::current().id()), "a job ran on the main thread")?;
        check(ids[0] != ids[1], "two jobs that met at a barrier ran on the same thread")
    });
}
