use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Lock {
    /// `None` once `into_name` has taken the name out: there is nothing left to release.
    name: Option<String>,
    log: Log,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock {
        log.borrow_mut().push(format!("acquire {name}"));
        Lock { name: Some(name.to_string()), log: Rc::clone(log) }
    }

    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or_default()
    }

    pub fn into_name(mut self) -> String {
        // `self.name` cannot be moved out of a type that implements `Drop` (E0509).
        // Take it and leave `None` behind, so that `drop` logs nothing.
        self.name.take().unwrap_or_default()
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        if let Some(name) = &self.name {
            self.log.borrow_mut().push(format!("release {name}"));
        }
    }
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
