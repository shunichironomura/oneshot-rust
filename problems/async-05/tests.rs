use std::cell::RefCell;
use std::pin::{Pin, pin};
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

use crate::judge::{case, check_eq};
use crate::user::sum_all;

type BoxFuture = Pin<Box<dyn Future<Output = u32>>>;
type Log = Rc<RefCell<Vec<(u32, &'static str)>>>;

/// Polls `fut` with a waker that does nothing until it is ready.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..1000 {
        if let Poll::Ready(out) = fut.as_mut().poll(&mut cx) {
            return out;
        }
    }
    panic!("the future did not finish after 1000 polls");
}

/// Returns `Pending` the first time it is polled and `Ready(())` after that.
struct YieldOnce {
    yielded: bool,
}

impl Future for YieldOnce {
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

fn ready(value: u32) -> BoxFuture {
    Box::pin(async move { value })
}

/// Logs when it starts and when it finishes, and is not ready on its first poll.
fn slow(id: u32, value: u32, log: &Log) -> BoxFuture {
    let log = Rc::clone(log);
    Box::pin(async move {
        log.borrow_mut().push((id, "start"));
        YieldOnce { yielded: false }.await;
        log.borrow_mut().push((id, "end"));
        value
    })
}

pub fn run() {
    case("example-1", || check_eq(block_on(sum_all(vec![ready(1), ready(2), ready(3)])), 6));
    case("case-1", || check_eq(block_on(sum_all(Vec::new())), 0));
    case("case-2", || {
        // One at a time, in the order of the vector, even when a future is not ready at once.
        let log = Log::default();
        let futs = vec![slow(0, 10, &log), ready(5), slow(1, 20, &log), slow(2, 30, &log)];
        check_eq(block_on(sum_all(futs)), 65)?;
        check_eq(
            log.take(),
            vec![(0, "start"), (0, "end"), (1, "start"), (1, "end"), (2, "start"), (2, "end")],
        )
    });
    case("case-3", || {
        // The future of sum_all can be passed to sum_all itself.
        let log = Log::default();
        let futs: Vec<BoxFuture> = vec![
            Box::pin(sum_all(vec![slow(0, 100, &log), ready(20)])),
            ready(4),
            slow(1, 3, &log),
        ];
        check_eq(block_on(sum_all(futs)), 127)?;
        check_eq(log.take(), vec![(0, "start"), (0, "end"), (1, "start"), (1, "end")])
    });
}
