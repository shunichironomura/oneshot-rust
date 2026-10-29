use std::future::{Future, IntoFuture};
use std::pin::pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};

use crate::judge::{case, check, check_eq};
use crate::user::{Delay, block_on, countdown, poll_count};

/// A waker that counts how many times it was woken.
struct CountingWaker(AtomicUsize);

impl CountingWaker {
    fn new() -> Arc<Self> {
        Arc::new(CountingWaker(AtomicUsize::new(0)))
    }

    fn count(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
}

impl Wake for CountingWaker {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

pub fn run() {
    case("example-1", || {
        check_eq(poll_count(countdown(3)), (3, 4))?;
        check_eq(block_on(Delay::new(2).extra(1)), 3)
    });
    case("example-2", || {
        check_eq(block_on(async { countdown(2).await + Delay::new(1).await }), 3)?;
        check_eq(poll_count(async { 5 }), (5, 1))
    });
    case("case-1", || {
        // Zero polls to wait: ready on the first poll.
        check_eq(poll_count(countdown(0)), (0, 1))?;
        check_eq(block_on(countdown(0)), 0)?;
        check_eq(block_on(Delay::new(0).extra(0)), 0)?;
        check_eq(poll_count(countdown(20)), (20, 21))
    });
    case("case-2", || {
        // Exactly one wake before each Pending, none on Ready.
        let counter = CountingWaker::new();
        let waker = Waker::from(counter.clone());
        let mut cx = Context::from_waker(&waker);
        let mut fut = pin!(countdown(3));
        for i in 1..=3 {
            check(fut.as_mut().poll(&mut cx).is_pending(), "expected Pending")?;
            check_eq(counter.count(), i)?;
        }
        check_eq(fut.as_mut().poll(&mut cx), Poll::Ready(3))?;
        check_eq(counter.count(), 3)
    });
    case("case-3", || {
        // The waker of the current poll is the one to wake; countdown(0) never wakes.
        let a = CountingWaker::new();
        let b = CountingWaker::new();
        let waker_a = Waker::from(a.clone());
        let waker_b = Waker::from(b.clone());
        let mut fut = pin!(countdown(3));
        check(fut.as_mut().poll(&mut Context::from_waker(&waker_a)).is_pending(), "expected Pending")?;
        check(fut.as_mut().poll(&mut Context::from_waker(&waker_b)).is_pending(), "expected Pending")?;
        check(fut.as_mut().poll(&mut Context::from_waker(&waker_a)).is_pending(), "expected Pending")?;
        check_eq((a.count(), b.count()), (2, 1))?;
        check_eq(fut.as_mut().poll(&mut Context::from_waker(&waker_b)), Poll::Ready(3))?;
        check_eq((a.count(), b.count()), (2, 1))?;
        let mut zero = pin!(countdown(0));
        check_eq(zero.as_mut().poll(&mut Context::from_waker(&waker_a)), Poll::Ready(0))?;
        check_eq(a.count(), 2)
    });
    case("case-4", || {
        // Every call to `extra` adds to the count.
        check_eq(block_on(Delay::new(1).extra(2).extra(3)), 6)?;
        check_eq(poll_count(Delay::new(2).extra(3).into_future()), (5, 6))?;
        check_eq(block_on(Delay::new(4)), 4)
    });
    case("case-5", || {
        check_eq(
            poll_count(async {
                countdown(1).await;
                countdown(1).await;
            }),
            ((), 3),
        )
    });
    case("case-6", || {
        // Nested async blocks that borrow a local across an await.
        let words = vec!["a", "bb", "ccc"];
        let fut = async {
            let inner = async { countdown(2).await * 10 }.await;
            let len = words.len() as u32;
            let extra = Delay::new(0).extra(len).await;
            inner + extra + words[2].len() as u32
        };
        check_eq(poll_count(fut), (26, 6))?;
        check_eq(block_on(async { block_on(countdown(2)) + countdown(1).await }), 3)
    });
}
