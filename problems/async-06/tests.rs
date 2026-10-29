use std::pin::pin;
use std::task::{Context, Poll, Waker};

use crate::judge::{case, check_eq};
use crate::user::{Tree, depth};

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

fn node(children: Vec<Tree>) -> Tree {
    Tree { children }
}

fn leaf() -> Tree {
    node(Vec::new())
}

/// `n` nodes in a line, so the depth is `n`.
fn chain(n: usize) -> Tree {
    let mut tree = leaf();
    for _ in 1..n {
        tree = node(vec![tree]);
    }
    tree
}

pub fn run() {
    case("example-1", || {
        let tree = node(vec![node(vec![leaf()]), leaf()]);
        check_eq(block_on(depth(&tree)), 3)
    });
    case("case-1", || {
        check_eq(block_on(depth(&leaf())), 1)?;
        check_eq(block_on(depth(&chain(10))), 10)
    });
    case("case-2", || {
        // The deepest branch can be anywhere among the children.
        let first = node(vec![chain(3), leaf(), chain(2)]);
        let middle = node(vec![leaf(), node(vec![leaf(), chain(2)]), leaf()]);
        let last = node(vec![leaf(), chain(2), node(vec![leaf(), chain(3)])]);
        check_eq(block_on(depth(&first)), 4)?;
        check_eq(block_on(depth(&middle)), 4)?;
        check_eq(block_on(depth(&last)), 5)
    });
}
