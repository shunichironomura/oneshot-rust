use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::judge::{case, check_eq};
use crate::user::Queue;

/// Adds 1 to a shared counter when dropped.
struct Counted<'a> {
    id: u32,
    drops: &'a Cell<u32>,
}

impl Drop for Counted<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

fn s(x: &str) -> String {
    String::from(x)
}

/// `peek()`, `peek_back()` and `len()` of a queue of strings.
fn ends(q: &Queue<String>) -> (Option<&str>, Option<&str>, usize) {
    (q.peek().map(String::as_str), q.peek_back().map(String::as_str), q.len())
}

/// A queue holding the given strings, built inside a function and returned by value.
fn queue_of(items: &[&str]) -> Queue<String> {
    let mut q = Queue::new();
    for &x in items {
        q.push(s(x));
    }
    q
}

pub fn run() {
    case("example-1", || {
        let mut q = Queue::new();
        q.push(s("a"));
        q.push(s("b"));
        q.push(s("c"));
        check_eq(ends(&q), (Some("a"), Some("c"), 3))?;
        check_eq(q.pop(), Some(s("a")))?;
        check_eq(q.pop(), Some(s("b")))?;
        q.push(s("d"));
        check_eq(ends(&q), (Some("c"), Some("d"), 2))?;
        check_eq(q.pop(), Some(s("c")))?;
        check_eq(q.pop(), Some(s("d")))?;
        check_eq(q.pop(), None)?;
        check_eq(ends(&q), (None, None, 0))
    });
    case("example-2", || {
        let item = Rc::new(7);
        let mut q = Queue::new();
        for _ in 0..3 {
            q.push(Rc::clone(&item));
        }
        check_eq(Rc::strong_count(&item), 4)?;
        let first = q.pop();
        check_eq(first.as_deref(), Some(&7))?;
        drop(q);
        check_eq(Rc::strong_count(&item), 2)
    });
    case("case-1", || {
        // Empty queues, and a queue that is emptied and then used again.
        let q: Queue<String> = Queue::new();
        check_eq(ends(&q), (None, None, 0))?;
        drop(q);
        let mut q = Queue::new();
        check_eq(q.pop(), None)?;
        q.push(s("x"));
        check_eq(ends(&q), (Some("x"), Some("x"), 1))?;
        check_eq(q.pop(), Some(s("x")))?;
        check_eq(ends(&q), (None, None, 0))?;
        check_eq(q.pop(), None)?;
        q.push(s("y"));
        q.push(s("z"));
        check_eq(ends(&q), (Some("y"), Some("z"), 2))?;
        check_eq(q.pop(), Some(s("y")))?;
        check_eq(ends(&q), (Some("z"), Some("z"), 1))?;
        check_eq(q.pop(), Some(s("z")))?;
        q.push(s("w"));
        check_eq(ends(&q), (Some("w"), Some("w"), 1))
    });
    case("case-2", || {
        // The last node becomes the first one, then the queue grows again.
        let mut q = Queue::new();
        q.push(1);
        q.push(2);
        check_eq(q.pop(), Some(1))?;
        check_eq((q.peek(), q.peek_back(), q.len()), (Some(&2), Some(&2), 1))?;
        q.push(3);
        check_eq((q.peek(), q.peek_back(), q.len()), (Some(&2), Some(&3), 2))?;
        for x in 4..12 {
            q.push(x);
            check_eq(q.pop(), Some(x - 2))?;
            check_eq(q.pop(), Some(x - 1))?;
            check_eq(q.peek_back(), Some(&x))?;
            q.push(x + 1000);
            check_eq(q.pop(), Some(x))?;
            check_eq(q.pop(), Some(x + 1000))?;
            q.push(x - 1);
            q.push(x);
        }
        check_eq((q.peek(), q.peek_back(), q.len()), (Some(&10), Some(&11), 2))
    });
    case("case-3", || {
        // Interleaved pushes and pops, compared with VecDeque.
        let mut q = Queue::new();
        let mut model = VecDeque::new();
        let mut next = 0u32;
        for round in 0..24 {
            for _ in 0..round % 4 + 1 {
                q.push(next);
                model.push_back(next);
                next += 1;
            }
            for _ in 0..round % 3 + 1 {
                check_eq(q.pop(), model.pop_front())?;
            }
            check_eq((q.peek(), q.peek_back(), q.len()), (model.front(), model.back(), model.len()))?;
        }
        while let Some(x) = model.pop_front() {
            check_eq(q.pop(), Some(x))?;
        }
        check_eq((q.pop(), q.len()), (None, 0))
    });
    case("case-4", || {
        // Each element is dropped exactly once: by the caller after `pop`, or by the queue.
        let drops = Cell::new(0);
        let mut q = Queue::new();
        for id in 0..6 {
            q.push(Counted { id, drops: &drops });
        }
        check_eq(drops.get(), 0)?;
        check_eq(q.pop().map(|c| c.id), Some(0))?;
        check_eq(drops.get(), 1)?;
        let kept = q.pop();
        check_eq(kept.as_ref().map(|c| c.id), Some(1))?;
        check_eq(q.peek().map(|c| c.id), Some(2))?;
        check_eq(q.peek_back().map(|c| c.id), Some(5))?;
        check_eq((drops.get(), q.len()), (1, 4))?;
        drop(q);
        check_eq(drops.get(), 5)?;
        drop(kept);
        check_eq(drops.get(), 6)?;
        // A queue that was emptied by `pop` owns nothing any more.
        let mut q = Queue::new();
        q.push(Counted { id: 7, drops: &drops });
        check_eq(q.pop().map(|c| c.id), Some(7))?;
        drop(q);
        check_eq(drops.get(), 7)
    });
    case("case-5", || {
        // Queues are moved around: returned from functions and stored in other queues.
        let mut a = queue_of(&["a"]);
        check_eq(ends(&a), (Some("a"), Some("a"), 1))?;
        a.push(s("b"));
        check_eq(ends(&a), (Some("a"), Some("b"), 2))?;
        let mut outer = Queue::new();
        outer.push(queue_of(&["x"]));
        outer.push(a);
        outer.push(queue_of(&[]));
        outer.push(queue_of(&["p", "q", "r"]));
        let mut first = outer.pop().unwrap();
        check_eq(ends(&first), (Some("x"), Some("x"), 1))?;
        first.push(s("y"));
        check_eq(ends(&first), (Some("x"), Some("y"), 2))?;
        let mut second = outer.pop().unwrap();
        check_eq(second.pop(), Some(s("a")))?;
        let moved = second;
        check_eq(ends(&moved), (Some("b"), Some("b"), 1))?;
        check_eq(outer.peek_back().map(ends), Some((Some("p"), Some("r"), 3)))?;
        check_eq(outer.len(), 2)?;
        // `outer` still holds two queues (one of them empty) when it is dropped.
        Ok(())
    });
}
