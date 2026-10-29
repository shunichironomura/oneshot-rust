use std::cell::RefCell;
use std::future::poll_fn;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};
use std::thread;

use crate::judge::{case, check_eq};
use crate::user::Executor;

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

/// A shared place where a waiting task leaves a clone of its waker.
#[derive(Clone, Default)]
struct Slot(Rc<RefCell<Option<Waker>>>);

impl Slot {
    fn put(&self, waker: &Waker) {
        *self.0.borrow_mut() = Some(waker.clone());
    }

    fn take(&self) -> Waker {
        self.0.borrow_mut().take().expect("no waker in the slot")
    }

    fn wake_by_ref(&self) {
        self.0.borrow().as_ref().expect("no waker in the slot").wake_by_ref();
    }
}

/// Leaves its waker in the slot and returns `Pending` without waking anything the first time it
/// is polled, `Ready` after that.
struct Park {
    slot: Slot,
    parked: bool,
}

impl Future for Park {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.parked {
            Poll::Ready(())
        } else {
            self.parked = true;
            self.slot.put(cx.waker());
            Poll::Pending
        }
    }
}

fn park(slot: &Slot) -> Park {
    Park { slot: slot.clone(), parked: false }
}

/// Logs `name`, then `times` times parks in `slot` and logs `name` again.
fn waiter(name: &'static str, times: usize, slot: Slot, log: Log) -> impl Future<Output = ()> {
    async move {
        log.borrow_mut().push(name);
        for _ in 0..times {
            park(&slot).await;
            log.borrow_mut().push(name);
        }
    }
}

pub fn run() {
    case("example-1", || {
        let log = Log::default();
        let mut ex = Executor::new();
        let l = log.clone();
        ex.spawn(async move {
            l.borrow_mut().push("a1");
            yield_now().await;
            l.borrow_mut().push("a2");
        });
        let l = log.clone();
        ex.spawn(async move { l.borrow_mut().push("b1") });
        check_eq(ex.run(), 3)?;
        check_eq(log.take(), vec!["a1", "b1", "a2"])
    });
    case("example-2", || {
        let slot = Slot::default();
        let mut ex = Executor::new();
        ex.spawn(park(&slot));
        check_eq(ex.run(), 1)?;
        check_eq(ex.run(), 0)?;
        slot.take().wake();
        check_eq(ex.run(), 1)
    });
    case("case-1", || {
        // An empty executor; spawn does not poll; spawning again after run.
        let log = Log::default();
        let mut ex = Executor::new();
        check_eq(ex.run(), 0)?;
        let l = log.clone();
        ex.spawn(async move { l.borrow_mut().push("x") });
        check_eq(log.borrow().len(), 0)?;
        check_eq(ex.run(), 1)?;
        check_eq(ex.run(), 0)?;
        let l = log.clone();
        ex.spawn(async move { l.borrow_mut().push("y") });
        check_eq(ex.run(), 1)?;
        check_eq(log.take(), vec!["x", "y"])
    });
    case("case-2", || {
        // A finished task is never polled again, and waking it does nothing, also when other
        // tasks are ready after it.
        let mut ex = Executor::new();
        ex.spawn(poll_fn(|cx| {
            cx.waker().wake_by_ref();
            Poll::Ready(())
        }));
        ex.spawn(yield_now());
        check_eq(ex.run(), 3)?;
        let old = Slot::default();
        let s = old.clone();
        ex.spawn(poll_fn(move |cx| {
            s.put(cx.waker());
            Poll::Ready(())
        }));
        check_eq(ex.run(), 1)?;
        let slot = Slot::default();
        ex.spawn(park(&slot));
        check_eq(ex.run(), 1)?;
        let old_waker = old.take();
        old_waker.wake_by_ref();
        check_eq(ex.run(), 0)?;
        // Woken before a waiting task: the waiting task still runs.
        old_waker.wake();
        slot.take().wake();
        check_eq(ex.run(), 1)
    });
    case("case-3", || {
        // A waker woken on another thread, and a waker kept from an earlier poll.
        let log = Log::default();
        let slot = Slot::default();
        let kept = slot.clone();
        let l = log.clone();
        let mut polls = 0;
        let mut ex = Executor::new();
        ex.spawn(poll_fn(move |cx| {
            polls += 1;
            if polls == 1 {
                kept.put(cx.waker());
            }
            if polls < 3 {
                Poll::Pending
            } else {
                l.borrow_mut().push("done");
                Poll::Ready(())
            }
        }));
        let other = Slot::default();
        ex.spawn(park(&other));
        check_eq(ex.run(), 2)?;
        let waker = slot.take();
        let copy = waker.clone();
        thread::scope(|s| {
            s.spawn(move || copy.wake());
        });
        check_eq(ex.run(), 1)?;
        check_eq(log.borrow().len(), 0)?;
        waker.wake();
        check_eq(ex.run(), 1)?;
        check_eq(log.take(), vec!["done"])
    });
    case("case-4", || {
        // First in, first out, whether a task was spawned or woken.
        let log = Log::default();
        let slots = [Slot::default(), Slot::default(), Slot::default()];
        let mut ex = Executor::new();
        for (name, slot) in ["p", "q", "r"].into_iter().zip(&slots) {
            ex.spawn(waiter(name, 1, slot.clone(), log.clone()));
        }
        check_eq(ex.run(), 3)?;
        slots[2].take().wake();
        ex.spawn(waiter("s", 0, Slot::default(), log.clone()));
        slots[0].take().wake();
        slots[1].take().wake();
        check_eq(ex.run(), 4)?;
        check_eq(log.take(), vec!["p", "q", "r", "r", "s", "p", "q"])
    });
    case("case-5", || {
        // A task woken during another task's poll goes last.
        let log = Log::default();
        let (a, b, c) = (Slot::default(), Slot::default(), Slot::default());
        let mut ex = Executor::new();
        ex.spawn(waiter("a", 3, a.clone(), log.clone()));
        let (l, a2, b2, c2) = (log.clone(), a.clone(), b.clone(), c.clone());
        ex.spawn(async move {
            l.borrow_mut().push("b");
            park(&b2).await;
            l.borrow_mut().push("b");
            c2.take().wake();
            a2.take().wake();
        });
        ex.spawn(waiter("c", 1, c.clone(), log.clone()));
        check_eq(ex.run(), 3)?;
        a.wake_by_ref();
        b.take().wake();
        check_eq(ex.run(), 4)?;
        check_eq(log.take(), vec!["a", "b", "c", "a", "b", "c", "a"])
    });
    case("case-6", || {
        // Finished futures are dropped at once, the others with the executor. A waker may
        // outlive the executor.
        let token = Rc::new(());
        let slot = Slot::default();
        let mut ex = Executor::new();
        let t = Rc::clone(&token);
        let mut yielded = false;
        ex.spawn(poll_fn(move |cx| {
            let _alive = &t;
            if yielded {
                return Poll::Ready(());
            }
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }));
        let (t, s) = (Rc::clone(&token), slot.clone());
        ex.spawn(async move {
            let _alive = t;
            park(&s).await;
        });
        check_eq(ex.run(), 3)?;
        check_eq(Rc::strong_count(&token), 2)?;
        drop(ex);
        check_eq(Rc::strong_count(&token), 1)?;
        slot.take().wake();
        Ok(())
    });
}
