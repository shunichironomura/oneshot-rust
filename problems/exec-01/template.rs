use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct Executor {
    // Add the fields you need.
}

impl Executor {
    pub fn new() -> Self {
        todo!()
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        todo!()
    }

    pub fn run(&mut self) -> usize {
        todo!()
    }

    pub fn pending(&self) -> usize {
        todo!()
    }
}
