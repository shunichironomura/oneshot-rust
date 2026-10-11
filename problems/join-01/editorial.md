`std::future::poll_fn` turns a closure `FnMut(&mut Context<'_>) -> Poll<T>` into a future whose
`poll` calls the closure. Thanks to `move`, the closure owns the state of the join: the inputs, and
for each input an `Option` that receives its output when it completes. `None` also means "still
running", which is what keeps a completed input from being polled again. Each poll first polls the
inputs that are still running (`poll_input`), and only then checks whether all the outputs are
there.

Compiling this on the first try takes care in four places:

- **Pinning.** `Future::poll` takes `self: Pin<&mut Self>`. `Pin::new(&mut a)` only works when
  `A: Unpin` ("`A` cannot be unpinned", E0277), and the inputs are generic; in the tests they are
  `async` blocks, which are not `Unpin`. Adding `A: Unpin` to the signature only moves the error
  into the harness. `Box::pin(a)` moves the input to the heap, where it never moves again, and
  `a.as_mut()` reborrows the `Pin<Box<A>>` as the `Pin<&mut A>` that `poll` takes. The
  `Pin<Box<A>>` itself is `Unpin`, so the closure can own it.
- **`move`.** Without it, the closure borrows the local variables of `join`, which are dropped when
  `join` returns. The compiler then reports that `a` does not live long enough, and also that "the
  parameter type `A` may not live long enough", suggesting `A: 'static`. Do not follow that
  suggestion: the inputs of the harness borrow local variables.
- **Taking the outputs.** `poll_fn` calls the closure once per poll, so the closure is `FnMut` and
  cannot move its captured variables out: `a_out.unwrap()` is rejected with "cannot move out of
  `a_out`, a captured variable in an `FnMut` closure" (E0507). `Option::take` moves the output out
  and leaves `None` behind.
- **No `Clone`.** `vec![None; n]` clones its element, so it requires `F::Output: Clone`. Build the
  vector of `None`s with `map(|_| None)` instead.

The behavior has traps of its own:

- **Poll every input that is still running.** `&&` is fine for the final check on the outputs, but
  `poll_a() && poll_b()` and `iter_mut().all(...)` short-circuit: they stop at the first input that
  returns `Pending`, and so does an early `return Poll::Pending`. `async move { (a.await, b.await) }`
  compiles, but it runs the inputs one after the other. In the first example, all of these log `a0`,
  `a1`, `b0`, `b1`, `b2` and take four polls.
- **Poll first, then decide.** A closure that first checks whether all the outputs are there and only
  then polls returns `Pending` from the poll in which the last input completes. Nothing woke the
  waker during that poll, so a real executor would never poll the joined future again.
- **Pass `cx` on.** The inputs wake the waker of the `Context` they are polled with; that is how the
  executor learns that the joined future can make progress. Polling them with a waker of your own,
  such as `Waker::noop()`, loses those wake-ups. Waking `cx.waker()` in the joined future itself is
  not needed, and it makes the executor poll the joined future again for nothing.
- **Keep the order.** The outputs go in input order, not in the order the inputs complete. Removing
  completed inputs with `swap_remove` also goes wrong: it moves the last input into the gap, so the
  remaining inputs are no longer polled in index order.

### Alternatives

A struct that implements `Future` itself works too, with one catch. `poll` gets `Pin<&mut Self>`,
and changing the fields needs a `&mut Self`, which `get_mut` (or `DerefMut`) provides only when
`Self: Unpin`. An auto trait such as `Unpin` holds for a type only if it holds for every field.
`Pin<Box<F>>` is always `Unpin`, but `F::Output` may not be, so without help the compiler reports
"`<A as Future>::Output` cannot be unpinned" at `get_mut`. The struct never pins anything it stores
inline (the inputs are behind `Pin<Box<_>>`, and the outputs are only kept), so it may promise
`Unpin` regardless. Implementing `Unpin` is safe; only pin projection would need `unsafe`.

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

/// One input of a join: running (pinned on the heap), then completed with its output.
enum Slot<F: Future> {
    Running(Pin<Box<F>>),
    Done(F::Output),
    Taken,
}

// The future is behind a `Box` and the output is never pinned, so a `Slot` may move freely
// even if `F::Output` is not `Unpin`.
impl<F: Future> Unpin for Slot<F> {}

impl<F: Future> Slot<F> {
    fn new(fut: F) -> Self {
        Slot::Running(Box::pin(fut))
    }

    /// Polls the input unless it has completed. Returns whether its output is available.
    fn poll(&mut self, cx: &mut Context<'_>) -> bool {
        if let Slot::Running(fut) = self {
            match fut.as_mut().poll(cx) {
                Poll::Ready(out) => *self = Slot::Done(out),
                Poll::Pending => return false,
            }
        }
        true
    }

    fn take(&mut self) -> F::Output {
        match std::mem::replace(self, Slot::Taken) {
            Slot::Done(out) => out,
            _ => unreachable!("the input has not completed"),
        }
    }
}

struct Join<A: Future, B: Future> {
    a: Slot<A>,
    b: Slot<B>,
}

impl<A: Future, B: Future> Future for Join<A, B> {
    type Output = (A::Output, B::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // `get_mut` needs `Join<A, B>: Unpin`, which holds because both slots are `Unpin`.
        let this = self.get_mut();
        let a_done = this.a.poll(cx);
        let b_done = this.b.poll(cx);
        if a_done && b_done { Poll::Ready((this.a.take(), this.b.take())) } else { Poll::Pending }
    }
}

pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)> {
    Join { a: Slot::new(a), b: Slot::new(b) }
}

struct JoinAll<F: Future> {
    slots: Vec<Slot<F>>,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = Vec<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut all_done = true;
        for slot in &mut this.slots {
            all_done &= slot.poll(cx);
        }
        if all_done {
            Poll::Ready(this.slots.iter_mut().map(Slot::take).collect())
        } else {
            Poll::Pending
        }
    }
}

pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>> {
    JoinAll { slots: futs.into_iter().map(Slot::new).collect() }
}
```

`join` can also pin its inputs without a heap allocation. An `async` block is pinned while it runs,
so `pin!` can pin values stored in the block itself. The closure then only borrows the outputs, and
after the `.await`, when the closure is gone, they can be moved out with `unwrap`. This replaces
`join` of the model solution (add `use std::pin::pin;`). It does not work for `join_all`: `pin!`
pins one value, and the elements of a `Vec` cannot be pinned in place without `unsafe`.

```rust
pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)> {
    async move {
        // The `async` block is pinned while it runs, so it can pin its own locals.
        let mut a = pin!(a);
        let mut b = pin!(b);
        let mut a_out = None;
        let mut b_out = None;
        poll_fn(|cx| {
            if a_out.is_none()
                && let Poll::Ready(out) = a.as_mut().poll(cx)
            {
                a_out = Some(out);
            }
            if b_out.is_none()
                && let Poll::Ready(out) = b.as_mut().poll(cx)
            {
                b_out = Some(out);
            }
            if a_out.is_some() && b_out.is_some() { Poll::Ready(()) } else { Poll::Pending }
        })
        .await;
        // The closure and its borrows are gone, so the outputs can be moved out.
        (a_out.unwrap(), b_out.unwrap())
    }
}
```

`join_all` can drop each input as soon as it completes. `retain_mut` keeps the inputs that are still
running, in order, and each output is stored together with the index of its input and sorted into
place at the end. No `Option` is needed, and `std::mem::take` gets the collected outputs out of the
`FnMut` closure. This replaces `join_all` of the model solution.

```rust
pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>> {
    let mut running: Vec<(usize, Pin<Box<F>>)> =
        futs.into_iter().map(Box::pin).enumerate().collect();
    let mut done: Vec<(usize, F::Output)> = Vec::new();
    poll_fn(move |cx| {
        // `retain_mut` keeps the remaining inputs in order and drops the completed ones.
        running.retain_mut(|(index, fut)| match fut.as_mut().poll(cx) {
            Poll::Ready(out) => {
                done.push((*index, out));
                false
            }
            Poll::Pending => true,
        });
        if running.is_empty() {
            let mut done = std::mem::take(&mut done);
            done.sort_by_key(|&(index, _)| index);
            Poll::Ready(done.into_iter().map(|(_, out)| out).collect())
        } else {
            Poll::Pending
        }
    })
}
```
