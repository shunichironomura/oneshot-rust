use std::future::poll_fn;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Polls `fut` unless it has already completed, that is, unless `out` already holds its output.
fn poll_input<F: Future>(fut: &mut Pin<Box<F>>, out: &mut Option<F::Output>, cx: &mut Context<'_>) {
    if out.is_none()
        && let Poll::Ready(value) = fut.as_mut().poll(cx)
    {
        *out = Some(value);
    }
}

pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)> {
    // `poll` needs `Pin<&mut A>`, and `A` may not be `Unpin`: pin both inputs on the heap.
    let mut a = Box::pin(a);
    let mut b = Box::pin(b);
    let mut a_out = None;
    let mut b_out = None;
    poll_fn(move |cx| {
        poll_input(&mut a, &mut a_out, cx);
        poll_input(&mut b, &mut b_out, cx);
        if a_out.is_some() && b_out.is_some() {
            // The closure is `FnMut`, so it cannot move its captures out: take the outputs.
            Poll::Ready((a_out.take().unwrap(), b_out.take().unwrap()))
        } else {
            Poll::Pending
        }
    })
}

pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>> {
    let mut futs: Vec<Pin<Box<F>>> = futs.into_iter().map(Box::pin).collect();
    // Not `vec![None; n]`, which would require `F::Output: Clone`.
    let mut outs: Vec<Option<F::Output>> = futs.iter().map(|_| None).collect();
    poll_fn(move |cx| {
        for (fut, out) in futs.iter_mut().zip(&mut outs) {
            poll_input(fut, out, cx);
        }
        if outs.iter().all(Option::is_some) {
            Poll::Ready(outs.iter_mut().map(|out| out.take().unwrap()).collect())
        } else {
            Poll::Pending
        }
    })
}
