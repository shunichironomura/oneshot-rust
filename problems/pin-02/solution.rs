use std::pin::Pin;
use std::task::{Context, Poll};

/// A future that polls its inner future at most a given number of times.
pub struct Budget<F> {
    /// The inner future, pinned on the heap. `None` once it has been dropped.
    inner: Option<Pin<Box<F>>>,
    /// How many more times the inner future may be polled.
    left: u32,
}

/// Wraps `fut` in a `Budget` that polls it at most `polls` times.
pub fn with_budget<F: Future>(fut: F, polls: u32) -> Budget<F> {
    // `Box::pin` moves the future to the heap, where it never moves again. The box itself may
    // move freely, so `Budget<F>` is `Unpin` even when `F` is not.
    Budget { inner: Some(Box::pin(fut)), left: polls }
}

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // `get_mut` needs `Budget<F>: Unpin`, which holds because both fields are `Unpin`.
        let this = self.get_mut();
        let output = if this.left == 0 {
            // A budget of 0: give up without polling.
            None
        } else {
            this.left -= 1;
            let fut = this.inner.as_mut().expect("a Budget is not polled after it is ready");
            // `as_mut` reborrows the `Pin<Box<F>>` as the `Pin<&mut F>` that `poll` takes.
            match fut.as_mut().poll(cx) {
                Poll::Ready(output) => Some(output),
                Poll::Pending if this.left > 0 => return Poll::Pending,
                // That was the last poll the budget allows: give up now, not on the next poll.
                Poll::Pending => None,
            }
        };
        // Finished or given up: drop the inner future now, not when the `Budget` is dropped.
        this.inner = None;
        Poll::Ready(output)
    }
}
