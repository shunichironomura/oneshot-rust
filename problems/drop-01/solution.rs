use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Lock {
    name: String,
    log: Log,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock {
        log.borrow_mut().push(format!("acquire {name}"));
        Lock { name: name.to_string(), log: Rc::clone(log) }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("release {}", self.name));
    }
}

pub fn with_lock<R>(name: &str, log: &Log, f: impl FnOnce() -> R) -> R {
    // A named variable holds the lock until the end of the function, after `f()` has run.
    // `let _ = ...` would release it at once: `_` binds nothing.
    let _lock = Lock::acquire(name, log);
    f()
}

pub fn release_all(mut locks: Vec<Lock>) {
    // Dropping a `Vec` drops its elements from the first to the last, so pop them from the end.
    while let Some(lock) = locks.pop() {
        drop(lock);
    }
}

pub fn swap_in(slot: &mut Lock, name: &str) {
    // The right-hand side runs first, then the assignment drops the old lock.
    *slot = Lock::acquire(name, &slot.log);
}

pub fn keep_first(locks: Vec<Lock>) -> Option<Lock> {
    // The iterator still owns the other locks, and drops them in order when it goes away.
    locks.into_iter().next()
}
