`poll` receives `self: Pin<&mut Self>`, not `&mut self`. Reading a field goes through `Deref`, but
assigning one goes through `DerefMut`, which `Pin<&mut T>` implements only when `T: Unpin`.
`Countdown` holds only `u32`s, so it is `Unpin` automatically. `deref_mut` borrows the `Pin` itself
mutably, though, so the parameter must be declared `mut self`; without it, `self.remaining -= 1`
fails with "cannot borrow `self` as mutable". (`self.get_mut()` is another way, see below.)

A future that returns `Pending` promises that its task will be woken when it is worth polling again;
a real executor sleeps until then. `Countdown` has nothing to wait for, so it wakes the current task
right away. Always use the waker of the `Context` you were given: the task may be polled with a
different waker the next time.

`block_on` first calls `into_future()`, which is also what `.await` does: that is why
`Delay::new(1).await` works, and since every `Future` is an `IntoFuture`, `block_on` accepts both.
Polling needs a `Pin<&mut F>`, and `Pin::new(&mut fut)` requires `F: Unpin`, so for a generic `F`
it is a compile error ("`F` cannot be unpinned"). Adding an `F: Unpin` bound is no way out: an
`async` block is not `Unpin` (it may hold references to its own locals across an `.await`), so the
tests that pass one would not compile. `std::pin::pin!` pins the value in place on the stack
instead.

`Future::poll` takes the `Pin<&mut F>` by value, so `fut.poll(&mut cx)` inside a loop moves `fut` in
the first iteration. `fut.as_mut()` reborrows it for one call. `Waker::noop()` (Rust 1.85) is a
ready-made `&'static Waker` that does nothing, and `Context::from_waker` wraps it.

### Alternatives

`Pin::get_mut` (also only for `Unpin` types) turns the pin back into a plain `&mut Self`.

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
    let this = self.get_mut();
    if this.remaining == 0 {
        return Poll::Ready(this.total);
    }
    this.remaining -= 1;
    cx.waker().wake_by_ref();
    Poll::Pending
}
```

`Box::pin` pins the future on the heap, and before `Waker::noop` existed, a no-op waker was made from
an `Arc` of a type that implements `Wake`.

```rust
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct NoopWaker;

impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {}
}

pub fn poll_count<F: Future>(fut: F) -> (F::Output, usize) {
    let mut fut = Box::pin(fut);
    let waker = Waker::from(Arc::new(NoopWaker));
    let mut cx = Context::from_waker(&waker);
    let mut polls = 0;
    loop {
        polls += 1;
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return (output, polls),
            Poll::Pending => {}
        }
    }
}
```

The loop can also live in `block_on`, with `poll_count` wrapping the future in another future that
counts its polls. A generic `F` may not be `Unpin`, but `Pin<Box<F>>` always is, so the wrapper is
`Unpin` and can update its counter through `DerefMut` just like `Countdown`.

```rust
pub fn block_on<F: IntoFuture>(f: F) -> F::Output {
    let mut fut = pin!(f.into_future());
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output;
        }
    }
}

struct Counted<F> {
    fut: Pin<Box<F>>,
    polls: usize,
}

impl<F: Future> Future for Counted<F> {
    type Output = (F::Output, usize);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls += 1;
        let polls = self.polls;
        self.fut.as_mut().poll(cx).map(|output| (output, polls))
    }
}

pub fn poll_count<F: Future>(fut: F) -> (F::Output, usize) {
    block_on(Counted { fut: Box::pin(fut), polls: 0 })
}
```
