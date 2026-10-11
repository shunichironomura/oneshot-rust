use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    // Add the fields you need.
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        todo!()
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }

    pub fn shutdown(self) -> usize {
        todo!()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        todo!()
    }
}
