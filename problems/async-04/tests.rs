use std::pin::pin;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};
use std::thread;

use crate::judge::{case, check, check_eq};
use crate::user::{forward_all, record_all};

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

fn shared(items: &[&str]) -> Mutex<Vec<String>> {
    Mutex::new(strings(items))
}

fn contents(m: &Mutex<Vec<String>>) -> Vec<String> {
    m.lock().unwrap().clone()
}

/// Polls `fut` with a waker that does nothing until it is ready.
/// Returns the output and the number of polls.
fn block_on<F: Future>(fut: F) -> (F::Output, usize) {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    for polls in 1..=1000 {
        if let Poll::Ready(out) = fut.as_mut().poll(&mut cx) {
            return (out, polls);
        }
    }
    panic!("the future did not finish after 1000 polls");
}

/// Requires the future to be `Send`, so that it can be moved to another thread.
fn sendable<F: Future + Send>(fut: F) -> F {
    fut
}

/// Polls `a` and `b` in turns (a, b, a, b, ...), skipping the one that is already ready,
/// until both are ready.
fn join<A: Future, B: Future>(a: A, b: B) -> (A::Output, B::Output) {
    let mut a = pin!(a);
    let mut b = pin!(b);
    let mut cx = Context::from_waker(Waker::noop());
    let mut out_a = None;
    let mut out_b = None;
    for _ in 0..1000 {
        if out_a.is_none()
            && let Poll::Ready(out) = a.as_mut().poll(&mut cx)
        {
            out_a = Some(out);
        }
        if out_b.is_none()
            && let Poll::Ready(out) = b.as_mut().poll(&mut cx)
        {
            out_b = Some(out);
        }
        if out_a.is_some() && out_b.is_some() {
            break;
        }
    }
    let a = out_a.expect("the first future did not finish");
    let b = out_b.expect("the second future did not finish");
    (a, b)
}

pub fn run() {
    case("sample-1", || {
        let log = shared(&[]);
        let (len, polls) = block_on(record_all(&log, strings(&["a", "b"])));
        check_eq(len, 2)?;
        check_eq(contents(&log), ["a", "b"])?;
        check(polls == 3, "record_all should return Pending once after each push")
    });
    case("sample-2", || {
        let inbox = shared(&["x", "y"]);
        let log = shared(&["start"]);
        let (len, _) = block_on(forward_all(&inbox, &log));
        check_eq(len, 3)?;
        check_eq(contents(&log), ["start", "x", "y"])?;
        check(contents(&inbox).is_empty(), "the inbox should be empty")
    });
    case("hidden-1", || {
        // Nothing to push: the current length, without yielding. Otherwise one yield per item.
        let log = shared(&["p", "q", "r"]);
        check_eq(block_on(record_all(&log, Vec::new())), (3, 1))?;
        check_eq(block_on(record_all(&log, strings(&["", "é"]))), (5, 3))?;
        check_eq(contents(&log), ["p", "q", "r", "", "é"])?;
        let inbox = shared(&[]);
        check_eq(block_on(forward_all(&inbox, &log)), (5, 1))?;
        inbox.lock().unwrap().extend(strings(&["b", "c", "d"]));
        check_eq(block_on(forward_all(&inbox, &log)), (8, 4))?;
        check_eq(contents(&log), ["p", "q", "r", "", "é", "b", "c", "d"])?;
        check(contents(&inbox).is_empty(), "the inbox should be empty")
    });
    case("hidden-2", || {
        // Each item is pushed before the yield, and the log is unlocked while the future waits.
        let log = shared(&["p"]);
        let mut fut = pin!(record_all(&log, strings(&["q1", "q2"])));
        let mut cx = Context::from_waker(Waker::noop());
        check(fut.as_mut().poll(&mut cx).is_pending(), "expected Pending")?;
        check_eq(contents(&log), ["p", "q1"])?;
        log.lock().unwrap().push("h".to_string());
        check(fut.as_mut().poll(&mut cx).is_pending(), "expected Pending")?;
        check_eq(contents(&log), ["p", "q1", "h", "q2"])?;
        check_eq(fut.as_mut().poll(&mut cx), Poll::Ready(4))
    });
    case("hidden-3", || {
        // Two calls take turns; each resolves to the length after its own last push.
        let log = shared(&[]);
        let got = join(
            record_all(&log, strings(&["a1", "a2"])),
            record_all(&log, strings(&["b1", "b2", "b3"])),
        );
        check_eq(got, (3, 5))?;
        check_eq(contents(&log), ["a1", "b1", "a2", "b2", "b3"])
    });
    case("hidden-4", || {
        // forward_all takes the messages all at once and does not keep the inbox locked:
        // a message pushed while it is recording stays in the inbox.
        let inbox = shared(&["m1", "m2"]);
        let log = shared(&["s"]);
        let got = join(forward_all(&inbox, &log), record_all(&inbox, strings(&["late"])));
        check_eq(got, (3, 1))?;
        check_eq(contents(&log), ["s", "m1", "m2"])?;
        check_eq(contents(&inbox), ["late"])
    });
    case("hidden-5", || {
        // Two futures created here run on another thread, taking turns.
        let inbox = shared(&["m1", "m2"]);
        let log = shared(&[]);
        let rec = sendable(record_all(&log, strings(&["a1", "a2", "a3"])));
        let fwd = sendable(forward_all(&inbox, &log));
        let got = thread::scope(|s| s.spawn(move || join(rec, fwd)).join().unwrap());
        check_eq(got, (5, 4))?;
        check_eq(contents(&log), ["a1", "m1", "a2", "m2", "a3"])?;
        check(contents(&inbox).is_empty(), "the inbox should be empty")
    });
}
