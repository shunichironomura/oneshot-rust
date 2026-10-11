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
    Budget { inner: Some(fut), left: polls }
}

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: this is a pin projection. `inner` is structurally pinned: it is only used through
        // the `Pin` made from it below, so it is never moved, and `Pin::set` drops it in place.
        // `left` is not pinned: a plain `&mut u32` cannot move the inner future. `Budget` has no
        // `Drop` impl and no `Unpin` impl of its own, so nothing else can move `inner` either.
        let this = unsafe { self.get_unchecked_mut() };
        let mut inner = unsafe { Pin::new_unchecked(&mut this.inner) };
        // `as_pin_mut` turns the `Pin<&mut Option<F>>` into an `Option<Pin<&mut F>>`.
        let output = match inner.as_mut().as_pin_mut() {
            Some(fut) if this.left > 0 => {
                this.left -= 1;
                match fut.poll(cx) {
                    Poll::Ready(output) => Some(output),
                    Poll::Pending if this.left > 0 => return Poll::Pending,
                    // That was the last poll the budget allows: give up now, not on the next poll.
                    Poll::Pending => None,
                }
            }
            // A budget of 0: give up without polling.
            _ => None,
        };
        // Finished or given up: drop the inner future now, in place, without moving it.
        inner.set(None);
        Poll::Ready(output)
    }
}
