use std::cell::Cell;
use std::marker::PhantomPinned;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::Poll::{Pending, Ready};
use std::task::{Context, Poll, Wake, Waker};

use crate::judge::{case, check_eq};
use crate::user::{Stream, StreamExt, iter, zip};

/// Counts how many times it was woken.
struct CountingWaker(AtomicUsize);

impl Wake for CountingWaker {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

/// Calls `poll` until it is ready, and calls it again only after the task was woken.
/// Returns the output and the number of calls.
fn drive<T>(mut poll: impl FnMut(&mut Context<'_>) -> Poll<T>) -> Result<(T, usize), String> {
    let counter = Arc::new(CountingWaker(AtomicUsize::new(0)));
    let waker = Waker::from(Arc::clone(&counter));
    let mut cx = Context::from_waker(&waker);
    for polls in 1..=100 {
        let wakes = counter.0.load(Ordering::SeqCst);
        if let Ready(output) = poll(&mut cx) {
            return Ok((output, polls));
        }
        if counter.0.load(Ordering::SeqCst) == wakes {
            return Err(format!("poll {polls} returned Pending, but nothing will wake the task"));
        }
    }
    Err("still Pending after 100 polls".to_string())
}

/// Runs a future to completion. Returns the output and the number of polls.
fn block_on<F: Future>(fut: F) -> Result<(F::Output, usize), String> {
    let mut fut = pin!(fut);
    drive(|cx| fut.as_mut().poll(cx))
}

/// All the items of `s`, read with `next`.
async fn collect<S: Stream + Unpin>(mut s: S) -> Vec<S::Item> {
    let mut items = Vec::new();
    while let Some(item) = s.next().await {
        items.push(item);
    }
    items
}

/// Polls a stream until it ends, and returns the result of every poll.
fn trace<S: Stream>(s: S) -> Result<Vec<Poll<Option<S::Item>>>, String> {
    let mut s = pin!(s);
    let mut results = Vec::new();
    drive(|cx| {
        loop {
            let result = s.as_mut().poll_next(cx);
            let (pending, ended) = (result.is_pending(), matches!(result, Ready(None)));
            results.push(result);
            if pending {
                return Pending;
            }
            if ended {
                return Ready(());
            }
        }
    })?;
    Ok(results)
}

/// Returns `Pending` (after waking the task) before each item and before the end.
/// Counts its polls in `polls`, and panics if it is polled after the end.
struct Slow<'a, T> {
    items: std::slice::Iter<'a, T>,
    waited: bool,
    ended: bool,
    polls: &'a Cell<usize>,
}

fn slow<'a, T>(items: &'a [T], polls: &'a Cell<usize>) -> Slow<'a, T> {
    Slow { items: items.iter(), waited: false, ended: false, polls }
}

impl<T: Copy> Stream for Slow<'_, T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        let this = self.get_mut();
        assert!(!this.ended, "a stream was polled after its end");
        this.polls.set(this.polls.get() + 1);
        if !this.waited {
            this.waited = true;
            cx.waker().wake_by_ref();
            return Pending;
        }
        this.waited = false;
        let item = this.items.next().copied();
        this.ended = item.is_none();
        Ready(item)
    }
}

/// Counts the polls of the stream it wraps in `polls`, and panics if it is polled after the end.
struct Counted<'a, S> {
    inner: S,
    ended: bool,
    polls: &'a Cell<usize>,
}

fn counted<S>(inner: S, polls: &Cell<usize>) -> Counted<'_, S> {
    Counted { inner, ended: false, polls }
}

impl<S: Stream + Unpin> Stream for Counted<'_, S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        let this = self.get_mut();
        assert!(!this.ended, "a stream was polled after its end");
        this.polls.set(this.polls.get() + 1);
        let result = Pin::new(&mut this.inner).poll_next(cx);
        this.ended = matches!(result, Ready(None));
        result
    }
}

/// Counts down from `n` to 1. It is not `Unpin`.
struct PinnedCountdown {
    n: u32,
    _pinned: PhantomPinned,
}

impl Iterator for PinnedCountdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.n;
        self.n = n.saturating_sub(1);
        (n > 0).then_some(n)
    }
}

pub fn run() {
    case("example-1", || {
        check_eq(
            trace(iter(vec![1, 2, 3]))?,
            vec![Ready(Some(1)), Ready(Some(2)), Ready(Some(3)), Ready(None)],
        )?;
        let mut s = iter(["x"]);
        check_eq(block_on(s.next())?, (Some("x"), 1))?;
        check_eq(block_on(s.next())?, (None, 1))
    });
    case("example-2", || {
        let (a_polls, b_polls) = (Cell::new(0), Cell::new(0));
        let z = zip(counted(iter(1..=3), &a_polls), slow(&['a', 'b'], &b_polls));
        check_eq(
            trace(z)?,
            vec![Pending, Ready(Some((1, 'a'))), Pending, Ready(Some((2, 'b'))), Pending, Ready(None)],
        )?;
        check_eq((a_polls.get(), b_polls.get()), (3, 6))
    });
    case("case-1", || {
        // An empty iterator, borrowed items, any `IntoIterator`; `iter` is never `Pending`.
        let polls = Cell::new(0);
        check_eq(block_on(collect(counted(iter(Vec::<u8>::new()), &polls)))?, (vec![], 1))?;
        check_eq(polls.get(), 1)?;
        let words = vec![String::from("ab"), String::from("c")];
        let expected = vec![Ready(Some(&words[0])), Ready(Some(&words[1])), Ready(None)];
        check_eq(trace(iter(&words))?, expected)?;
        let mut s = iter([10, 20]);
        check_eq(block_on(s.next())?, (Some(10), 1))?;
        check_eq(block_on(collect(s))?, (vec![20], 1))
    });
    case("case-2", || {
        // Each poll of the future of `next` polls the stream once; `next` works on trait objects.
        let polls = Cell::new(0);
        let mut s = slow(&[5, 6], &polls);
        check_eq(block_on(s.next())?, (Some(5), 2))?;
        check_eq(block_on(s.next())?, (Some(6), 2))?;
        check_eq(block_on(s.next())?, (None, 2))?;
        check_eq(polls.get(), 6)?;
        let polls = Cell::new(0);
        let mut boxed: Box<dyn Stream<Item = u32> + Unpin + '_> = Box::new(slow(&[9], &polls));
        check_eq(block_on(boxed.next())?, (Some(9), 2))?;
        check_eq(block_on(boxed.next())?, (None, 2))?;
        check_eq(polls.get(), 4)?;
        let mut it = iter(vec![7, 8]);
        let dynamic: &mut (dyn Stream<Item = i32> + Unpin) = &mut it;
        check_eq(block_on(dynamic.next())?, (Some(7), 1))?;
        check_eq(block_on(collect(it))?, (vec![8], 1))
    });
    case("case-3", || {
        // `b` is not polled while `a` is `Pending`, nor after `a` has ended.
        let (a_polls, b_polls) = (Cell::new(0), Cell::new(0));
        let z = zip(slow(&[1, 2], &a_polls), counted(iter(["x", "y", "z"]), &b_polls));
        check_eq(
            trace(z)?,
            vec![Pending, Ready(Some((1, "x"))), Pending, Ready(Some((2, "y"))), Pending, Ready(None)],
        )?;
        check_eq((a_polls.get(), b_polls.get()), (6, 2))
    });
    case("case-4", || {
        // Either stream can end first, also before the first pair.
        let (a_polls, b_polls) = (Cell::new(0), Cell::new(0));
        let z = zip(counted(iter(Vec::<i32>::new()), &a_polls), slow(&['q'], &b_polls));
        check_eq(trace(z)?, vec![Ready(None)])?;
        check_eq((a_polls.get(), b_polls.get()), (1, 0))?;
        let (a_polls, b_polls) = (Cell::new(0), Cell::new(0));
        let z = zip(counted(iter(vec![1, 2]), &a_polls), slow::<char>(&[], &b_polls));
        check_eq(trace(z)?, vec![Pending, Ready(None)])?;
        check_eq((a_polls.get(), b_polls.get()), (1, 2))?;
        let (a_polls, b_polls) = (Cell::new(0), Cell::new(0));
        let z = zip(counted(iter(vec![1, 2, 3]), &a_polls), counted(iter("ab".chars()), &b_polls));
        check_eq(trace(z)?, vec![Ready(Some((1, 'a'))), Ready(Some((2, 'b'))), Ready(None)])?;
        check_eq((a_polls.get(), b_polls.get()), (3, 3))
    });
    case("case-5", || {
        // Both streams are slow: the item of `a` is held while `b` is `Pending`.
        let (a_polls, b_polls) = (Cell::new(0), Cell::new(0));
        let z = zip(slow(&[1, 2, 3], &a_polls), slow(&['p', 'q', 'r', 's'], &b_polls));
        check_eq(
            trace(z)?,
            vec![
                Pending,
                Pending,
                Ready(Some((1, 'p'))),
                Pending,
                Pending,
                Ready(Some((2, 'q'))),
                Pending,
                Pending,
                Ready(Some((3, 'r'))),
                Pending,
                Ready(None),
            ],
        )?;
        check_eq((a_polls.get(), b_polls.get()), (8, 6))?;
        // Nested `zip`s.
        let (a_polls, b_polls, c_polls) = (Cell::new(0), Cell::new(0), Cell::new(0));
        let ab = zip(counted(iter(1..=3), &a_polls), slow(&['a', 'b', 'c'], &b_polls));
        let z = zip(ab, slow(&["x", "y"], &c_polls));
        check_eq(block_on(collect(z))?, (vec![((1, 'a'), "x"), ((2, 'b'), "y")], 7))?;
        check_eq((a_polls.get(), b_polls.get(), c_polls.get()), (3, 6, 6))
    });
    case("case-6", || {
        // An iterator that is not `Unpin`, and items that are not `Unpin` (`async` blocks).
        check_eq(
            trace(iter(PinnedCountdown { n: 3, _pinned: PhantomPinned }))?,
            vec![Ready(Some(3)), Ready(Some(2)), Ready(Some(1)), Ready(None)],
        )?;
        let polls = Cell::new(0);
        let futures = iter((1..=3).map(|n| async move { n * 10 }));
        let results = trace(zip(futures, slow(&['x', 'y'], &polls)))?;
        let pending: Vec<bool> = results.iter().map(|r| r.is_pending()).collect();
        check_eq(pending, vec![true, false, true, false, true, false])?;
        let mut pairs = Vec::new();
        for result in results {
            if let Ready(Some((fut, name))) = result {
                pairs.push((block_on(fut)?.0, name));
            }
        }
        check_eq(pairs, vec![(10, 'x'), (20, 'y')])?;
        let futures = block_on(collect(iter((1..=2).map(|n| async move { n + 100 }))))?.0;
        let mut outputs = Vec::new();
        for fut in futures {
            outputs.push(block_on(fut)?.0);
        }
        check_eq(outputs, vec![101, 102])
    });
}
