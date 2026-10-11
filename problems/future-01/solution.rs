use std::future::{Future, IntoFuture};
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

pub struct Countdown {
    total: u32,
    remaining: u32,
}

pub fn countdown(n: u32) -> Countdown {
    Countdown { total: n, remaining: n }
}

impl Future for Countdown {
    type Output = u32;

    // `mut self`: assigning to a field goes through `DerefMut` of the `Pin`,
    // which is available because `Countdown` is `Unpin`.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        if self.remaining == 0 {
            return Poll::Ready(self.total);
        }
        self.remaining -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

pub struct Delay {
    polls: u32,
}

impl Delay {
    pub fn new(polls: u32) -> Self {
        Delay { polls }
    }

    pub fn extra(self, k: u32) -> Self {
        Delay { polls: self.polls + k }
    }
}

impl IntoFuture for Delay {
    type Output = u32;
    type IntoFuture = Countdown;

    fn into_future(self) -> Countdown {
        countdown(self.polls)
    }
}

pub fn block_on<F: IntoFuture>(f: F) -> F::Output {
    poll_count(f.into_future()).0
}

pub fn poll_count<F: Future>(fut: F) -> (F::Output, usize) {
    // An async block may not be moved once it has been polled, so pin it in place.
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    let mut polls = 0;
    loop {
        polls += 1;
        // `as_mut` reborrows the pinned reference, so it can be used again next time.
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return (output, polls);
        }
    }
}
