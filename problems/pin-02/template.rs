use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A future that polls its inner future at most a given number of times.
pub struct Budget<F> {
    // Replace this placeholder with the fields you need.
    _todo: PhantomData<F>,
}

/// Wraps `fut` in a `Budget` that polls it at most `polls` times.
pub fn with_budget<F: Future>(fut: F, polls: u32) -> Budget<F> {
    todo!()
}

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
