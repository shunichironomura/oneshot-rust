use std::pin::pin;
use std::task::{Context, Poll, Waker};

use crate::judge::{case, check_eq};
use crate::user::greet;

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

pub fn run() {
    case("example-1", || check_eq(block_on(greet("Ann".to_string())), "Hello, Ann!"));
    case("case-1", || {
        // The futures are awaited after greet has returned, in the opposite order.
        let empty = greet(String::new());
        let spaced = greet(" 世界 ".to_string());
        check_eq(block_on(spaced), "Hello,  世界 !")?;
        check_eq(block_on(empty), "Hello, !")
    });
}
