use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// A spawned task: its future, and the waker to poll it with. Use it or not, as you like.
#[allow(dead_code)]
struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    waker: Waker,
}

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
}
