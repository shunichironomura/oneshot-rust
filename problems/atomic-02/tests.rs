use std::thread;

use crate::judge::{case, check, check_eq};
use crate::user::SpinLock;

static LOG: SpinLock<Vec<u32>> = SpinLock::new(Vec::new());

/// Calls `try_lock` `attempts` times, adding 1 under the lock each time it succeeds.
/// Returns the number of successful calls.
fn try_many(lock: &SpinLock<u32>, attempts: u32) -> u32 {
    let mut won = 0;
    for _ in 0..attempts {
        if let Some(mut guard) = lock.try_lock() {
            *guard += 1;
            won += 1;
        }
    }
    won
}

pub fn run() {
    case("sample-1", || {
        let counter = SpinLock::new(0);
        thread::scope(|s| {
            for _ in 0..3 {
                s.spawn(|| {
                    for _ in 0..5 {
                        *counter.lock() += 1;
                    }
                });
            }
        });
        check_eq(counter.into_inner(), 15)
    });
    case("sample-2", || {
        let lock = SpinLock::new(String::from("spin"));
        let mut guard = lock.lock();
        guard.push_str("lock");
        check(lock.try_lock().is_none(), "try_lock returned Some while a guard was alive")?;
        drop(guard);
        check_eq(lock.try_lock().map(|g| g.len()), Some(8))
    });
    case("hidden-1", || {
        // A lock in a static, built with the const `new`, filled by two threads.
        let push_all = |t: u32| {
            for i in 0..6 {
                LOG.lock().push(t * 10 + i);
            }
        };
        thread::scope(|s| {
            s.spawn(move || push_all(1));
            push_all(2);
        });
        let log = LOG.try_lock().map(|g| g.clone()).unwrap_or_default();
        check_eq(log.len(), 12)?;
        for t in 1..=2 {
            // Each thread's values arrive in the order it pushed them.
            let mine: Vec<u32> = log.iter().copied().filter(|v| v / 10 == t).collect();
            check_eq(mine, (0..6).map(|i| t * 10 + i).collect::<Vec<u32>>())?;
        }
        Ok(())
    });
    case("hidden-2", || {
        // try_lock on a free lock always succeeds; on a held lock it always fails.
        let lock = SpinLock::new(7u8);
        for _ in 0..20 {
            match lock.try_lock() {
                Some(mut guard) => *guard += 1,
                None => return Err("try_lock returned None on a free lock".to_string()),
            }
        }
        let held = lock.try_lock();
        check(held.is_some(), "try_lock returned None on a free lock")?;
        for _ in 0..3 {
            check(lock.try_lock().is_none(), "try_lock returned Some while a guard was alive")?;
        }
        drop(held);
        *lock.lock() += 1;
        check_eq(lock.into_inner(), 28)
    });
    case("hidden-3", || {
        // Two threads calling try_lock at the same time.
        let lock = SpinLock::new(0);
        let wins = thread::scope(|s| {
            let other = s.spawn(|| try_many(&lock, 80));
            try_many(&lock, 80) + other.join().unwrap()
        });
        check_eq(lock.into_inner(), wins)
    });
    case("hidden-4", || {
        // One thread calling lock while another calls try_lock.
        let lock = SpinLock::new(0);
        let wins = thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..40 {
                    *lock.lock() += 1;
                }
            });
            try_many(&lock, 80)
        });
        check_eq(lock.into_inner(), wins + 40)
    });
    case("hidden-5", || {
        // Replacing and changing a String through the guard.
        let lock = SpinLock::new(String::from("old"));
        {
            let mut guard = lock.lock();
            *guard = String::from("new");
            guard.make_ascii_uppercase();
            guard.push('!');
        }
        check_eq(lock.lock().as_str(), "NEW!")?;
        check_eq(lock.into_inner(), "NEW!")
    });
}
