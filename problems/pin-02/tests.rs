use std::cell::Cell;
use std::marker::PhantomPinned;
use std::pin::{Pin, pin};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};

use crate::judge::{case, check, check_eq};
use crate::user::{Budget, with_budget};

/// Counts the polls and the drops of the `Ticks` futures created with it.
#[derive(Default)]
struct Stats {
    polls: Cell<u32>,
    drops: Cell<u32>,
}

impl Stats {
    fn new() -> Rc<Self> {
        Rc::new(Stats::default())
    }

    /// `(polls, drops)` so far.
    fn get(&self) -> (u32, u32) {
        (self.polls.get(), self.drops.get())
    }
}

/// `ticks(n, out)` of the statement: a future that is not `Unpin`. It returns `Pending` on its
/// first `n` polls, waking the waker of the current poll each time, and `Ready(out)` on the next
/// one. It records its polls and its drop in `stats`. Unlike an `async` block, which drops its
/// captures as soon as it completes, it holds on to `stats` until it is dropped.
struct Ticks<T> {
    left: Cell<u32>,
    out: Cell<Option<T>>,
    stats: Rc<Stats>,
    _pinned: PhantomPinned,
}

fn ticks<T>(stats: &Rc<Stats>, n: u32, out: T) -> Ticks<T> {
    Ticks {
        left: Cell::new(n),
        out: Cell::new(Some(out)),
        stats: Rc::clone(stats),
        _pinned: PhantomPinned,
    }
}

impl<T> Future for Ticks<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        // The state is in `Cell`s, so a shared reference is enough.
        let this = self.into_ref().get_ref();
        this.stats.polls.set(this.stats.polls.get() + 1);
        match this.left.get() {
            0 => Poll::Ready(this.out.take().expect("a ticks future was polled after it was ready")),
            n => {
                this.left.set(n - 1);
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

impl<T> Drop for Ticks<T> {
    fn drop(&mut self) {
        self.stats.drops.set(self.stats.drops.get() + 1);
    }
}

/// An output that is not `Clone`, `Copy`, `Default` or `Unpin`.
#[derive(Debug, PartialEq)]
struct Ticket(usize, PhantomPinned);

fn ticket(n: usize) -> Ticket {
    Ticket(n, PhantomPinned)
}

/// A waker that counts how many times it was woken.
#[derive(Default)]
struct WakeCount(AtomicUsize);

impl WakeCount {
    fn get(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
}

impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

/// Runs `fut` on the current thread. Like a real executor, it polls the future again only after
/// its waker has been woken. Returns the output and the number of polls.
fn block_on<F: Future>(fut: F) -> (F::Output, usize) {
    let wakes = Arc::new(WakeCount::default());
    let waker = Waker::from(Arc::clone(&wakes));
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    for polls in 1..=50 {
        let before = wakes.get();
        if let Poll::Ready(out) = fut.as_mut().poll(&mut cx) {
            return (out, polls);
        }
        assert!(
            wakes.get() > before,
            "the future returned Pending without waking its waker, so it would never be polled again"
        );
    }
    panic!("the future was not ready after 50 polls");
}

/// Polls a `Budget` once with `waker`.
fn poll_budget<F: Future>(fut: Pin<&mut Budget<F>>, waker: &Waker) -> Poll<Option<F::Output>> {
    fut.poll(&mut Context::from_waker(waker))
}

pub fn run() {
    case("sample-1", || {
        let stats = Stats::new();
        let mut fut = pin!(with_budget(ticks(&stats, 2, "done"), 3));
        let waker = Waker::noop();
        check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending on poll 1")?;
        check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending on poll 2")?;
        check_eq(poll_budget(fut.as_mut(), waker), Poll::Ready(Some("done")))?;
        // (polls, drops) of the ticks future: dropped already, while the Budget still exists.
        check_eq(stats.get(), (3, 1))
    });
    case("sample-2", || {
        let stats = Stats::new();
        let mut fut = pin!(with_budget(ticks(&stats, 5, "late"), 3));
        let waker = Waker::noop();
        check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending on poll 1")?;
        check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending on poll 2")?;
        check_eq(poll_budget(fut.as_mut(), waker), Poll::Ready(None))?;
        check_eq(stats.get(), (3, 1))
    });
    case("hidden-1", || {
        // A budget of 0: the first poll gives up without polling, and drops the inner future.
        let wakes = Arc::new(WakeCount::default());
        let waker = Waker::from(Arc::clone(&wakes));
        let stats = Stats::new();
        let mut fut = pin!(with_budget(ticks(&stats, 0, 1), 0));
        check_eq(stats.get(), (0, 0))?;
        check_eq(poll_budget(fut.as_mut(), &waker), Poll::Ready(None))?;
        check_eq(stats.get(), (0, 1))?;
        // An async block that was never polled is dropped with the future it captured.
        let captured = ticks(&stats, 0, 2);
        let mut fut = pin!(with_budget(async move { captured.await * 10 }, 0));
        check_eq(stats.get(), (0, 1))?;
        check_eq(poll_budget(fut.as_mut(), &waker), Poll::Ready(None))?;
        check_eq(stats.get(), (0, 2))?;
        check_eq(wakes.get(), 0)
    });
    case("hidden-2", || {
        // Ready on the last allowed poll, or one poll short; a huge budget.
        let stats = Stats::new();
        check_eq(block_on(with_budget(ticks(&stats, 0, 'a'), 1)), (Some('a'), 1))?;
        check_eq(stats.get(), (1, 1))?;
        let stats = Stats::new();
        check_eq(block_on(with_budget(ticks(&stats, 3, 'b'), 3)), (None, 3))?;
        check_eq(stats.get(), (3, 1))?;
        let stats = Stats::new();
        check_eq(block_on(with_budget(ticks(&stats, 3, 'c'), 4)), (Some('c'), 4))?;
        check_eq(stats.get(), (4, 1))?;
        let stats = Stats::new();
        check_eq(block_on(with_budget(ticks(&stats, 6, 'd'), u32::MAX)), (Some('d'), 7))?;
        check_eq(stats.get(), (7, 1))?;
        let stats = Stats::new();
        check_eq(block_on(with_budget(ticks(&stats, 0, 'e'), 0)), (None, 1))?;
        check_eq(stats.get(), (0, 1))
    });
    case("hidden-3", || {
        // Ready with budget to spare: the inner future is dropped in that poll all the same.
        let waker = Waker::noop();
        let stats = Stats::new();
        let mut fut = pin!(with_budget(ticks(&stats, 1, 42), 10));
        check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending on poll 1")?;
        check_eq(stats.get(), (1, 0))?;
        check_eq(poll_budget(fut.as_mut(), waker), Poll::Ready(Some(42)))?;
        check_eq(stats.get(), (2, 1))?;
        // A Budget dropped before it is ready drops the inner future with it, polled or not.
        let stats = Stats::new();
        let fut = Box::pin(with_budget(ticks(&stats, 3, 0), 5));
        drop(fut);
        check_eq(stats.get(), (0, 1))?;
        let stats = Stats::new();
        let mut fut = Box::pin(with_budget(ticks(&stats, 3, 0), 5));
        check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending")?;
        check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending")?;
        check_eq(stats.get(), (2, 0))?;
        drop(fut);
        check_eq(stats.get(), (2, 1))
    });
    case("hidden-4", || {
        // The inner future gets the waker of the current poll; the Budget wakes nothing itself.
        let a = Arc::new(WakeCount::default());
        let b = Arc::new(WakeCount::default());
        let waker_a = Waker::from(Arc::clone(&a));
        let waker_b = Waker::from(Arc::clone(&b));
        let stats = Stats::new();
        let mut fut = pin!(with_budget(ticks(&stats, 4, 9), 3));
        check(poll_budget(fut.as_mut(), &waker_a).is_pending(), "expected Pending")?;
        check_eq((a.get(), b.get()), (1, 0))?;
        check(poll_budget(fut.as_mut(), &waker_b).is_pending(), "expected Pending")?;
        check_eq((a.get(), b.get()), (1, 1))?;
        check_eq(poll_budget(fut.as_mut(), &waker_a), Poll::Ready(None))?;
        check_eq((a.get(), b.get()), (2, 1))?;
        check_eq(stats.get(), (3, 1))?;
        let mut fut = pin!(with_budget(ticks(&stats, 2, 8), 5));
        check(poll_budget(fut.as_mut(), &waker_b).is_pending(), "expected Pending")?;
        check(poll_budget(fut.as_mut(), &waker_b).is_pending(), "expected Pending")?;
        check_eq((a.get(), b.get()), (2, 3))?;
        check_eq(poll_budget(fut.as_mut(), &waker_a), Poll::Ready(Some(8)))?;
        check_eq((a.get(), b.get()), (2, 3))?;
        check_eq(stats.get(), (6, 2))
    });
    case("hidden-5", || {
        // Async blocks that borrow locals and output a value that is not Unpin.
        let words = vec![String::from("ab"), String::from("cde")];
        let stats = Stats::new();
        let task = || async {
            let mut total = 0;
            for word in &words {
                total += ticks(&stats, word.len() as u32, word.len()).await;
            }
            ticket(total)
        };
        // The block needs 6 polls: 3 for "ab", the third of which starts "cde", and 3 more.
        check_eq(block_on(with_budget(task(), 6)), (Some(ticket(5)), 6))?;
        check_eq(stats.get(), (7, 2))?;
        // Cancelled while it awaits the second ticks future, which is dropped with the block.
        let stats_before = stats.get();
        let waker = Waker::noop();
        let mut fut = pin!(with_budget(task(), 5));
        for _ in 0..4 {
            check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending")?;
        }
        check_eq(stats.get(), (stats_before.0 + 5, stats_before.1 + 1))?;
        check_eq(poll_budget(fut.as_mut(), waker), Poll::Ready(None))?;
        check_eq(stats.get(), (stats_before.0 + 6, stats_before.1 + 2))
    });
    case("hidden-6", || {
        // Budgets nest, and they can be awaited inside async blocks.
        let stats = Stats::new();
        check_eq(block_on(with_budget(with_budget(ticks(&stats, 5, 1), 2), 4)), (Some(None), 2))?;
        check_eq(stats.get(), (2, 1))?;
        let stats = Stats::new();
        let waker = Waker::noop();
        let mut fut = pin!(with_budget(with_budget(ticks(&stats, 5, 1), 4), 2));
        check(poll_budget(fut.as_mut(), waker).is_pending(), "expected Pending")?;
        check_eq(poll_budget(fut.as_mut(), waker), Poll::Ready(None))?;
        check_eq(stats.get(), (2, 1))?;
        let stats = Stats::new();
        check_eq(block_on(with_budget(with_budget(ticks(&stats, 1, 7), 3), 3)), (Some(Some(7)), 2))?;
        check_eq(stats.get(), (2, 1))?;
        let stats = Stats::new();
        let fut = async {
            let first = with_budget(ticks(&stats, 1, 20), 5).await;
            let second = with_budget(ticks(&stats, 9, 1), 2).await;
            first.unwrap_or(0) + second.unwrap_or(0)
        };
        check_eq(block_on(fut), (20, 3))?;
        check_eq(stats.get(), (4, 2))
    });
}
