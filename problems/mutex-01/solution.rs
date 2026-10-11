use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Memo {
    cache: Mutex<BTreeMap<u64, u64>>,
    queue: Mutex<Vec<u64>>,
    misses: AtomicUsize,
}

impl Memo {
    pub fn new() -> Self {
        Memo {
            cache: Mutex::new(BTreeMap::new()),
            queue: Mutex::new(Vec::new()),
            misses: AtomicUsize::new(0),
        }
    }

    pub fn get_or_compute(&self, key: u64, f: impl FnOnce(&Memo, u64) -> u64) -> u64 {
        // The guard is a temporary of this `let` statement: it is dropped at the `;`.
        let cached = self.cache.lock().unwrap().get(&key).copied();
        if let Some(value) = cached {
            return value;
        }
        // No lock is held while `f` runs, so `f` may use this memo again.
        let value = f(self, key);
        self.cache.lock().unwrap().insert(key, value);
        self.misses.fetch_add(1, Ordering::Relaxed);
        value
    }

    pub fn misses(&self) -> usize {
        self.misses.load(Ordering::Relaxed)
    }

    pub fn schedule(&self, key: u64) {
        self.queue.lock().unwrap().push(key);
    }

    pub fn run_all(&self, f: impl Fn(&Memo, u64) -> u64) -> Vec<(u64, u64)> {
        let mut done = Vec::new();
        loop {
            // Lock the queue only to pop one key; the guard is dropped at the end of this statement.
            let Some(key) = self.queue.lock().unwrap().pop() else {
                break;
            };
            let value = self.get_or_compute(key, &f);
            done.push((key, value));
            if key > 1 {
                self.schedule(key / 2);
            }
        }
        done
    }
}

pub fn fib(m: &Memo, n: u64) -> u64 {
    m.get_or_compute(n, |m, n| if n < 2 { n } else { fib(m, n - 1) + fib(m, n - 2) })
}
