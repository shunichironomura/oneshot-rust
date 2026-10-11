use std::cell::RefCell;
use std::rc::Rc;

/// A handle to a value that several owners share.
#[derive(Clone, Default)]
pub struct Handle<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Handle { inner: Rc::new(RefCell::new(value)) }
    }

    /// Returns another handle to the same value.
    pub fn share(&self) -> Handle<T> {
        self.clone()
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.inner.borrow_mut());
    }

    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.inner.borrow())
    }

    pub fn handles(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

pub fn fan_out<T>(h: &Handle<T>, n: usize) -> Vec<Handle<T>> {
    (0..n).map(|_| h.clone()).collect()
}
