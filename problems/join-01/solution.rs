use std::future::poll_fn;
use std::pin::Pin;
use std::task::Poll;

pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>> {
    // `poll` needs `Pin<&mut F>`, and `F` may not be `Unpin`: pin each input on the heap.
    let mut futs: Vec<Pin<Box<F>>> = futs.into_iter().map(Box::pin).collect();
    // `None` until the input completes. Not `vec![None; n]`, which would require
    // `F::Output: Clone`.
    let mut outs: Vec<Option<F::Output>> = futs.iter().map(|_| None).collect();
    poll_fn(move |cx| {
        for (fut, out) in futs.iter_mut().zip(&mut outs) {
            // A completed input is never polled again.
            if out.is_none()
                && let Poll::Ready(value) = fut.as_mut().poll(cx)
            {
                *out = Some(value);
            }
        }
        if outs.iter().all(Option::is_some) {
            // The closure is `FnMut`, so it cannot move its captures out: take the outputs.
            Poll::Ready(outs.iter_mut().map(|out| out.take().unwrap()).collect())
        } else {
            Poll::Pending
        }
    })
}
