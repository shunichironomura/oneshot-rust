use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Countdown {
    // Add the fields you need.
}

pub fn countdown(n: u32) -> Countdown {
    todo!()
}

impl Future for Countdown {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        todo!()
    }
}

pub struct Delay {
    // Add the fields you need.
}

impl Delay {
    pub fn new(polls: u32) -> Self {
        todo!()
    }

    pub fn extra(self, k: u32) -> Self {
        todo!()
    }
}

impl IntoFuture for Delay {
    type Output = u32;
    type IntoFuture = Countdown;

    fn into_future(self) -> Countdown {
        todo!()
    }
}

pub fn block_on<F: IntoFuture>(f: F) -> F::Output {
    todo!()
}

pub fn poll_count<F: Future>(fut: F) -> (F::Output, usize) {
    todo!()
}
