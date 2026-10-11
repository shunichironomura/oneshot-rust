`poll` receives `self: Pin<&mut Self>`, and changing a field (the budget, or the inner future when
it is dropped) needs a `&mut Self`. `Pin` hands one out only when `Self: Unpin`, through `get_mut` or
through `DerefMut` behind `mut self`. `Unpin` is an auto trait: a struct is `Unpin` when all its
fields are. With the inner future stored inline, as in `inner: Option<F>`, `Budget<F>` is `Unpin`
only when `F` is, and the `async` blocks and `ticks` futures of the harness are not. The compiler
then rejects `self.inner = None` ("cannot assign to data in dereference of `Pin<&mut Budget<F>>`",
E0594), `self.get_mut()` and `Pin::new(&mut ...)` on the inner future ("`F` cannot be unpinned",
E0277). Adding `F: Unpin`, as the compiler suggests, only moves the error into the harness.

Keeping `F` inline would need *pin projection*: turning the `Pin<&mut Budget<F>>` into a
`Pin<&mut F>` for the field. That is `unsafe` (`Pin::map_unchecked_mut`, or a crate such as
`pin-project`), which this problem forbids. The safe way is to pin the inner future on the heap.
`Box::pin(fut)` returns a `Pin<Box<F>>`, which is `Unpin` whatever `F` is: moving the box moves only
the pointer, never the future. With the fields `Option<Pin<Box<F>>>` and `u32`, `Budget<F>` is
`Unpin` automatically, `get_mut` gives a plain `&mut Budget<F>`, and `fut.as_mut()` reborrows the
box as the `Pin<&mut F>` that `poll` takes.

The `Option` lets the `Budget` drop the inner future early: assigning `None` drops the box and the
future in it, right there in `poll`. In Rust, dropping a future is how it is cancelled. An `async`
block dropped in the middle of an `.await` never resumes, and the values it holds at that point (such
as the future it is awaiting) are dropped. Dropping the inner future matters even after it has
returned `Ready`. A finished `async` block has already dropped its local variables and captures, but
a hand-written future, such as `ticks` of the harness, keeps what it owns until it is dropped.

Count carefully. A budget of `0` means "do not poll at all", so check it before polling. After a
`Pending`, check it again: when the last allowed poll returns `Pending`, the `Budget` must give up in
that same poll. Returning `Pending` and giving up on the next poll costs the caller an extra poll and
keeps the inner future alive for longer than needed.

The `Budget` passes `cx` on unchanged. The inner future wakes the waker of the `Context` it is polled
with, and that is how the executor learns that the `Budget` can make progress; polling the inner
future with a waker of your own, such as `Waker::noop()`, loses those wake-ups. The `Budget` returns
`Pending` only right after the inner future did, so it never needs to wake anything itself.

### Alternatives

Pin a box that holds an `Option<F>` instead. `Option::as_pin_mut` turns a `Pin<&mut Option<F>>` into
an `Option<Pin<&mut F>>`, and `Pin::set` replaces the pinned value in place, dropping the old one:
`set(None)` drops the inner future without ever moving it. Pin projection of an inline `Option<F>`
would use the same two calls; the box makes them safe.

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Budget<F> {
    /// Pinned on the heap. `None` once the inner future has been dropped.
    inner: Pin<Box<Option<F>>>,
    left: u32,
}

pub fn with_budget<F: Future>(fut: F, polls: u32) -> Budget<F> {
    Budget { inner: Box::pin(Some(fut)), left: polls }
}

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        // `as_pin_mut`: `Pin<&mut Option<F>>` -> `Option<Pin<&mut F>>`.
        let output = match this.inner.as_mut().as_pin_mut() {
            Some(fut) if this.left > 0 => {
                this.left -= 1;
                match fut.poll(cx) {
                    Poll::Ready(output) => Some(output),
                    Poll::Pending if this.left > 0 => return Poll::Pending,
                    Poll::Pending => None,
                }
            }
            // A budget of 0: give up without polling.
            _ => None,
        };
        // Drops the inner future in place, without moving it.
        this.inner.set(None);
        Poll::Ready(output)
    }
}
```

Or take the box out of the `Option` for the duration of the poll, and put it back only when it is
still needed. Taking it out moves only the pointer, so this is fine for futures that are not
`Unpin`. When the box is not put back, it goes out of scope at the end of `poll`, which drops the
inner future before `poll` returns. This version also writes the fields through `DerefMut` of the
`Pin` (hence `mut self`), which, like `get_mut`, needs `Budget<F>: Unpin`. It replaces `poll` of the
model solution.

```rust
fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    // Moving the box moves only the pointer; the future stays where it is.
    let mut fut = self.inner.take().expect("a Budget is not polled after it is ready");
    if self.left == 0 {
        return Poll::Ready(None);
    }
    self.left -= 1;
    match fut.as_mut().poll(cx) {
        Poll::Ready(output) => Poll::Ready(Some(output)),
        Poll::Pending if self.left > 0 => {
            // Still needed: put it back.
            self.inner = Some(fut);
            Poll::Pending
        }
        Poll::Pending => Poll::Ready(None),
    }
    // Unless it was put back, `fut` is dropped here, before `poll` returns.
}
```
