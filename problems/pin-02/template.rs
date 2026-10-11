use std::pin::Pin;
use std::task::{Context, Poll};

/// A future that polls its inner future at most a given number of times.
pub struct Budget<F> {
    /// The inner future, stored inline. `None` once it has been dropped.
    inner: Option<F>,
    /// How many more times the inner future may be polled.
    left: u32,
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
