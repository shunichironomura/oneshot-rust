use std::future::poll_fn;
use std::pin::Pin;
use std::task::{Context, Poll};

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
        poll_fn(|cx| todo!())
    }
}

impl<S: Stream + ?Sized> StreamExt for S {}

/// A stream of the items of an iterator.
pub struct Iter<I> {
    iter: I,
}

pub fn iter<I: IntoIterator>(items: I) -> Iter<I::IntoIter> {
    todo!()
}

impl<I: Iterator> Stream for Iter<I> {
    type Item = I::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<I::Item>> {
        todo!()
    }
}

/// A stream of pairs of items of two streams.
pub struct Zip<A: Stream, B: Stream> {
    a: A,
    b: B,
    /// The item of `a` that waits for its partner from `b`.
    held: Option<A::Item>,
}

pub fn zip<A: Stream + Unpin, B: Stream + Unpin>(a: A, b: B) -> Zip<A, B> {
    todo!()
}

impl<A: Stream + Unpin, B: Stream + Unpin> Stream for Zip<A, B> {
    type Item = (A::Item, B::Item);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}

pub async fn collect<S: Stream + Unpin>(s: S) -> Vec<S::Item> {
    todo!()
}
