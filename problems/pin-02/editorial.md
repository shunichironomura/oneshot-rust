`poll` receives `self: Pin<&mut Self>`, and changing a field (the budget, or the inner future when
it is dropped) needs mutable access. `Pin` hands out a `&mut Self` only when `Self: Unpin`, through
`get_mut` or through `DerefMut` behind `mut self`. `Unpin` is an auto trait: `Budget<F>` keeps an
`Option<F>` inline, so it is `Unpin` only when `F` is, and the `async` blocks and `ticks` futures of
the harness are not. So the compiler rejects `self.left -= 1` ("cannot assign to data in dereference
of `Pin<&mut Budget<F>>`", E0594), `self.get_mut()`, and `Pin::new(&mut ...)` on the inner future
("`F` cannot be unpinned", E0277). Adding `F: Unpin`, as the compiler suggests, only moves the error
into the harness. Pinning the inner future on the heap with `Box::pin` would make `Budget<F>`
`Unpin`, but the statement rules that out: like the combinators of async libraries, a `Budget` does
not allocate.

What is left is *pin projection*: turning the `Pin<&mut Budget<F>>` into a `Pin<&mut Option<F>>`
for the field `inner`, and into a plain `&mut u32` for `left`. Rust has no safe projection for your
own types (the `pin-project` crate generates one), so it is written with `unsafe`:
`self.get_unchecked_mut()` gives the `&mut Budget<F>`, and `Pin::new_unchecked(&mut this.inner)`
pins the field again. The unsafe part is a promise that you choose, field by field, and keep:

- `inner` is *structurally pinned*: once it has been polled, it is never moved again. Not with
  `Option::take`, `mem::replace` or `mem::swap`, and not even to drop it: it must be dropped where
  it is. `Budget` must not implement `Unpin` (that would let safe code take a `&mut Budget<F>` out of
  a `Pin` and move the inner future), nor `Drop` with code that moves the field.
- `left` is not pinned. Handing out a `&mut u32` cannot move the inner future.

With the `Pin<&mut Option<F>>`, `as_pin_mut()` gives an `Option<Pin<&mut F>>` to poll, and
`set(None)` drops the inner future in place and stores `None`. `as_mut()` reborrows the pin, so that
it can still be used for `set` afterwards.

The trap is `take()`. With an `Option` field, the natural code takes the future out, polls it, and
puts it back, or takes it out to drop it. With `unsafe { Pin::new_unchecked(&mut fut) }` on the moved
future, this compiles, but it breaks the promise. An `async` block that borrows its own local across
an `.await` holds a pointer into itself, and after the move, it still points to the old place (in
one of the tests, the block awaits inside a `for x in &items` loop over its own local array). The
next poll then follows a dangling pointer, and Miri reports undefined behavior. The `ticks` futures
of the harness also check that they stay where they were first polled.

Dropping the inner future early is how a future is cancelled in Rust: an `async` block dropped in the
middle of an `.await` never resumes, and the values it holds at that point are dropped. It matters
even after `Ready`: a finished `async` block has already dropped its local variables and captures,
but a hand-written future, such as `ticks`, keeps what it owns until it is dropped.

Count carefully. A budget of `0` means "do not poll at all", so check it before polling. After a
`Pending`, check it again: when the last allowed poll returns `Pending`, the `Budget` gives up in
that same poll. Pass `cx` on unchanged: the inner future wakes the waker of the `Context` it is
polled with, which is how the executor learns that the `Budget` can make progress. The `Budget`
returns `Pending` only right after the inner future did, so it never needs to wake anything itself.

### Alternatives

Write the projection once, as a method that splits the pinned `Budget` into its fields. This is what
`pin-project` generates, and it keeps the `unsafe` code in one place.

```rust
impl<F> Budget<F> {
    /// The pin projection: `inner` is structurally pinned, `left` is not.
    fn project(self: Pin<&mut Self>) -> (Pin<&mut Option<F>>, &mut u32) {
        // SAFETY: `inner` is only ever used through the `Pin` returned here, so it is never
        // moved out of a pinned `Budget`. `left` is not pinned.
        let this = unsafe { self.get_unchecked_mut() };
        (unsafe { Pin::new_unchecked(&mut this.inner) }, &mut this.left)
    }
}

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (mut inner, left) = self.project();
        let mut output = None;
        if *left > 0
            && let Some(fut) = inner.as_mut().as_pin_mut()
        {
            *left -= 1;
            match fut.poll(cx) {
                Poll::Ready(value) => output = Some(value),
                Poll::Pending if *left > 0 => return Poll::Pending,
                Poll::Pending => {}
            }
        }
        inner.set(None);
        Poll::Ready(output)
    }
}
```

`Pin::set` is nothing more than an assignment, and an assignment drops the old value in place before
it writes the new one. So the inner future may also be dropped with `this.inner = None` on the
`&mut Budget<F>`, and pinned only for the call to `poll`. This replaces `poll` of the model solution.

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    // SAFETY: nothing below moves the inner future out of the pinned `Budget`.
    let this = unsafe { self.get_unchecked_mut() };
    let mut output = None;
    if this.left > 0
        && let Some(fut) = &mut this.inner
    {
        this.left -= 1;
        // SAFETY: `fut` lives inside the pinned `Budget` and is never moved.
        match unsafe { Pin::new_unchecked(fut) }.poll(cx) {
            Poll::Ready(value) => output = Some(value),
            Poll::Pending if this.left > 0 => return Poll::Pending,
            Poll::Pending => {}
        }
    }
    // Assigning drops the old value in place, so the inner future is not moved.
    this.inner = None;
    Poll::Ready(output)
}
```
