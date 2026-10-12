use std::future::poll_fn;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

/// An asynchronous sequence of items: the async counterpart of `Iterator`.
pub trait Stream {
    type Item;

    /// `Ready(Some(item))`: the next item. `Ready(None)`: the stream has ended.
    /// `Pending`: no item yet; the stream makes sure that the task is woken later.
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

pub trait StreamExt: Stream {
    /// Resolves to the next item of the stream, or to `None` once it has ended.
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + '_
    where
        Self: Unpin,
    {
        // `&mut *self` reborrows `self` for one call. Passing `self` itself would move it into
        // the first call, and the closure could not be called again.
        poll_fn(|cx| Pin::new(&mut *self).poll_next(cx))
    }
}

impl<S: Stream + ?Sized> StreamExt for S {}

/// A stream of the items of an iterator.
pub struct Iter<I> {
    iter: I,
}

// `Iter` never pins its iterator, so it may be moved even while it is pinned, whatever `I` is.
// Without this, `Iter<I>` would be `Unpin` only when `I` is, and `get_mut` would not compile.
impl<I> Unpin for Iter<I> {}

pub fn iter<I: IntoIterator>(items: I) -> Iter<I::IntoIter> {
    Iter { iter: items.into_iter() }
}

impl<I: Iterator> Stream for Iter<I> {
    type Item = I::Item;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<I::Item>> {
        Poll::Ready(self.get_mut().iter.next())
    }
}

/// A stream of pairs of items of two streams.
pub struct Zip<A: Stream, B: Stream> {
    a: A,
    b: B,
    /// The item of `a` that waits for its partner from `b`.
    held: Option<A::Item>,
}

// The held item is only moved in and out, never pinned, so `Zip` does not need it to be `Unpin`.
// (`a` and `b` are pinned with `Pin::new`, which the bounds `A: Unpin, B: Unpin` allow.)
impl<A: Stream, B: Stream> Unpin for Zip<A, B> {}

pub fn zip<A: Stream + Unpin, B: Stream + Unpin>(a: A, b: B) -> Zip<A, B> {
    Zip { a, b, held: None }
}

impl<A: Stream + Unpin, B: Stream + Unpin> Stream for Zip<A, B> {
    type Item = (A::Item, B::Item);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let a_item = match this.held.take() {
            Some(item) => item,
            // `ready!` returns `Pending` from `poll_next` if `a` is not ready.
            None => match ready!(Pin::new(&mut this.a).poll_next(cx)) {
                Some(item) => item,
                None => return Poll::Ready(None),
            },
        };
        match Pin::new(&mut this.b).poll_next(cx) {
            Poll::Ready(Some(b_item)) => Poll::Ready(Some((a_item, b_item))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => {
                // Keep the item of `a` for the next poll instead of dropping it.
                this.held = Some(a_item);
                Poll::Pending
            }
        }
    }
}
