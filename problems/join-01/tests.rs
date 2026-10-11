use std::cell::RefCell;
use std::future::{Ready, ready};
use std::marker::PhantomPinned;
use std::pin::{Pin, pin};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};

use crate::judge::{case, check, check_eq};
use crate::user::{join, join_all};

type Log = Rc<RefCell<Vec<&'static str>>>;

/// Wakes its task and returns `Pending` the first time it is polled, `Ready` after that.
struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

/// A task that logs its steps one at a time and yields once between two steps. It completes in
/// the poll that logs its last step (its first poll if there are no steps) and outputs the number
/// of steps. It is an `async` block, so it is not `Unpin`, and the `Rc` makes it `!Send`.
fn steps(log: &Log, names: &'static [&'static str]) -> impl Future<Output = usize> + use<> {
    let log = Rc::clone(log);
    async move {
        for (i, &name) in names.iter().enumerate() {
            if i > 0 {
                yield_now().await;
            }
            log.borrow_mut().push(name);
        }
        names.len()
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

/// Polls `fut` once with `waker`.
fn poll_with<F: Future>(fut: Pin<&mut F>, waker: &Waker) -> Poll<F::Output> {
    fut.poll(&mut Context::from_waker(waker))
}

pub fn run() {
    case("sample-1", || {
        let log = Log::default();
        let fut = join(steps(&log, &["a0", "a1"]), steps(&log, &["b0", "b1", "b2"]));
        check_eq(block_on(fut), ((2, 3), 3))?;
        check_eq(log.take(), ["a0", "b0", "a1", "b1", "b2"])
    });
    case("sample-2", || {
        let log = Log::default();
        let fut = join_all(vec![
            steps(&log, &["x0", "x1", "x2"]),
            steps(&log, &["y0"]),
            steps(&log, &["z0", "z1"]),
        ]);
        check_eq(block_on(fut), (vec![3, 1, 2], 3))?;
        check_eq(log.take(), ["x0", "y0", "z0", "x1", "z1", "x2"])
    });
    case("hidden-1", || {
        // No inputs, or only inputs that are ready at once: ready on the first poll.
        check_eq(block_on(join_all(Vec::<Ready<Ticket>>::new())), (Vec::new(), 1))?;
        check_eq(block_on(join(async { ticket(1) }, ready("two"))), ((ticket(1), "two"), 1))
    });
    case("hidden-2", || {
        // The inputs get the waker of the current poll; the joined future wakes nothing itself.
        let a = Arc::new(WakeCount::default());
        let b = Arc::new(WakeCount::default());
        let waker_a = Waker::from(Arc::clone(&a));
        let waker_b = Waker::from(Arc::clone(&b));
        let log = Log::default();
        let mut fut = pin!(join(steps(&log, &["a0", "a1"]), steps(&log, &["b0", "b1", "b2"])));
        check(poll_with(fut.as_mut(), &waker_a).is_pending(), "expected Pending")?;
        check_eq((a.get(), b.get()), (2, 0))?;
        check(poll_with(fut.as_mut(), &waker_b).is_pending(), "expected Pending")?;
        check_eq((a.get(), b.get()), (2, 1))?;
        check_eq(poll_with(fut.as_mut(), &waker_a), Poll::Ready((2, 3)))?;
        check_eq((a.get(), b.get()), (2, 1))?;
        let mut fut = pin!(join_all(vec![steps(&log, &["x0", "x1", "x2"]), steps(&log, &["y0"])]));
        check(poll_with(fut.as_mut(), &waker_b).is_pending(), "expected Pending")?;
        check_eq((a.get(), b.get()), (2, 2))?;
        check(poll_with(fut.as_mut(), &waker_a).is_pending(), "expected Pending")?;
        check_eq((a.get(), b.get()), (3, 2))?;
        check_eq(poll_with(fut.as_mut(), &waker_b), Poll::Ready(vec![3, 1]))?;
        check_eq((a.get(), b.get()), (3, 2))?;
        check_eq(log.take(), ["a0", "b0", "a1", "b1", "b2", "x0", "y0", "x1", "x2"])
    });
    case("hidden-3", || {
        // Inputs that borrow locals; outputs that are not Clone, Default or Unpin.
        let words = vec![String::from("ab"), String::from("c"), String::new()];
        let futs: Vec<_> = words
            .iter()
            .map(|w| async move {
                for _ in 0..w.len() {
                    yield_now().await;
                }
                ticket(w.len())
            })
            .collect();
        check_eq(block_on(join_all(futs)), (vec![ticket(2), ticket(1), ticket(0)], 3))?;
        let fut = join(async { ticket(words.len()) }, async { words[0].as_str() });
        check_eq(block_on(fut), ((ticket(3), "ab"), 1))
    });
    case("hidden-4", || {
        // Outputs in input order, whatever the order in which the inputs complete; nothing runs
        // before the first poll; joined futures can be joined again.
        let log = Log::default();
        let fut = join_all(vec![
            steps(&log, &["p0", "p1", "p2"]),
            steps(&log, &["q0"]),
            steps(&log, &["r0", "r1"]),
            steps(&log, &["s0", "s1", "s2"]),
        ]);
        check_eq(block_on(fut), (vec![3, 1, 2, 3], 3))?;
        check_eq(log.take(), ["p0", "q0", "r0", "s0", "p1", "r1", "s1", "p2", "s2"])?;
        let fut = join(
            join(steps(&log, &["a0", "a1"]), steps(&log, &["b0"])),
            join_all(vec![steps(&log, &["c0", "c1"]), steps(&log, &[])]),
        );
        check(log.borrow().is_empty(), "an input ran before the joined future was polled")?;
        check_eq(block_on(fut), (((2, 1), vec![2, 0]), 2))?;
        check_eq(log.take(), ["a0", "b0", "c0", "a1", "c1"])
    });
}
