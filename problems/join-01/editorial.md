`std::future::poll_fn` turns a closure `FnMut(&mut Context<'_>) -> Poll<T>` into a future whose
`poll` calls the closure. Thanks to `move`, the closure owns the state of the join: the inputs, and
for each input an `Option` that receives its output when it completes. `None` also means "still
running", which is what keeps a completed input from being polled again. Each poll first polls the
inputs that are still running, and only then checks whether all the outputs are there.

Compiling this on the first try takes care in four places:

- **Pinning.** `Future::poll` takes `self: Pin<&mut Self>`. `Pin::new(fut)` only works when
  `F: Unpin` ("`F` cannot be unpinned", E0277), and the inputs are generic; in the tests they are
  `async` blocks, which are not `Unpin`. Adding `F: Unpin` to the signature only moves the error
  into the harness. `Box::pin` moves each input to the heap, where it never moves again, and
  `fut.as_mut()` reborrows the `Pin<Box<F>>` as the `Pin<&mut F>` that `poll` takes. Moving the
  `Pin<Box<F>>` (into the `Vec`, into the closure, and later together with the joined future) moves
  only the pointer, never the input: `Pin<Box<F>>` is `Unpin`.
- **`move`.** Without it, the closure borrows the local variables of `join_all`, which are dropped
  when `join_all` returns. The compiler then reports that "the parameter type `F` may not live long
  enough", suggesting `F: 'static`, and also that `futs` does not live long enough. Do not follow
  that suggestion: the inputs of the harness borrow local variables.
- **Taking the outputs.** `poll_fn` calls the closure once per poll, so the closure is `FnMut` and
  cannot move its captured variables out: `outs.into_iter()` is rejected with "cannot move out of
  `outs`, a captured variable in an `FnMut` closure" (E0507). `Option::take` moves each output out
  and leaves `None` behind.
- **No `Clone`.** `vec![None; n]` clones its element, so it requires `F::Output: Clone`. Build the
  vector of `None`s with `map(|_| None)` instead.

The behavior has traps of its own:

- **Poll every input that is still running.** `all` is fine for the final check on the outputs,
  but polling inside `iter_mut().all(...)` short-circuits: it stops at the first input that returns
  `Pending`, and so does an early `return Poll::Pending`. An `async move` block that `.await`s each
  input in a loop compiles, but it runs the inputs one after the other. In the first example, such
  a `join_all` logs `a0`, `a1`, `b0`, `b1`, `b2` and takes four polls.
- **Poll first, then decide.** A closure that first checks whether all the outputs are there and
  only then polls returns `Pending` from the poll in which the last input completes. Nothing woke
  the waker during that poll, so a real executor would never poll the joined future again.
- **Pass `cx` on.** The inputs wake the waker of the `Context` they are polled with; that is how the
  executor learns that the joined future can make progress. Polling them with a waker of your own,
  such as `Waker::noop()`, loses those wake-ups. Waking `cx.waker()` in the joined future itself is
  not needed, and the statement rules it out: it makes the executor poll the joined future again
  for nothing.
- **Keep the order.** The outputs go in input order, not in the order the inputs complete. Removing
  completed inputs with `swap_remove` also goes wrong: it moves the last input into the gap, so the
  remaining inputs are no longer polled in index order.

### Alternatives

A struct that implements `Future` itself works too, with one catch. `poll` gets `Pin<&mut Self>`,
and changing the fields needs a `&mut Self`, which `get_mut` (or `DerefMut`) provides only when
`Self: Unpin`. An auto trait such as `Unpin` holds for a type only if it holds for every field.
`Pin<Box<F>>` is always `Unpin`, but `F::Output` may not be, so without help the compiler reports
"`<F as Future>::Output` cannot be unpinned" at `get_mut`. The struct never pins anything it stores
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

struct JoinAll<F: Future> {
    slots: Vec<Slot<F>>,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = Vec<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // `get_mut` needs `JoinAll<F>: Unpin`, which holds because every slot is `Unpin`.
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
