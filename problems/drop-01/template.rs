use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Lock {
    name: String,
    log: Log,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock {
        todo!()
    }

    pub fn name(&self) -> &str {
        todo!()
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        // Log the release here.
    }
}

pub fn with_lock<R>(name: &str, log: &Log, f: impl FnOnce() -> R) -> R {
    todo!()
}

pub fn release_all(locks: Vec<Lock>) {
    todo!()
}

pub fn swap_in(slot: &mut Lock, name: &str) {
    todo!()
}

pub fn keep_first(locks: Vec<Lock>) -> Option<Lock> {
    todo!()
}
