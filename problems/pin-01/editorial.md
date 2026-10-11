`Future::poll` takes `self: Pin<&mut Self>`, and `Pin::new` builds one only when the target is
`Unpin`, that is, a type that does not mind being moved, so pinning it promises nothing. The future
of an `async` block is not `Unpin`. It is a state machine that may keep references into itself
across an `.await` (to one of its own locals, for example), and moving it would leave them
dangling. A `dyn Future` is not `Unpin` either, since nothing is known about what is inside. So
`Pin::new(task.as_mut())` is rejected: `dyn Future<Output = u32>` cannot be unpinned (E0277).
Dropping `.as_mut()`, as the compiler suggests, only moves the error: `Box<F>` is a future only
when `F: Unpin`.

The fix is to pin each future when it is created: `Box::pin` instead of `Box::new`. `Box::pin`
moves the future to the heap and returns a `Pin<Box<_>>`, which promises that the future never
moves again. It coerces to `Pin<Box<dyn Future<Output = u32>>>` just like a `Box<_>` coerces to
`Box<dyn Future<Output = u32>>`. The `Pin<Box<_>>` itself is only a pointer and is `Unpin`, so it
can be moved freely: collected into a `Vec`, paired with its index, dropped by `retain_mut`. To
poll it, `task.as_mut()` (`Pin::as_mut`) reborrows the `&mut Pin<Box<_>>` as the
`Pin<&mut dyn Future<Output = u32>>` that `poll` takes, without giving up the box.

This fix touches every piece of the plumbing: the `Task` alias, both `Box::new` calls (including
the one cast with `as Task`), and the `poll` call. Changing only the alias leaves mismatched types
at the `Box::new` calls, and `Pin::new` still fails. `YieldNow` shows the other
side: it is a plain struct with a `bool`, so it is `Unpin`, and its own `poll` can assign
`self.yielded` through `Pin<&mut Self>`.

Other tempting fixes fail. With `Box<dyn Future<Output = u32> + Unpin>`, the `async` blocks
themselves no longer fit, because they are not `Unpin`. `std::pin::pin!` pins a value in a local
variable of the current block, so it cannot keep a run-time number of futures alive after the loop
that creates them.

### Alternatives

Make the executor generic and pin the futures inside it. Each function then passes a `Vec` of its
own `async` block type, and no trait object is needed. `Box::pin` is still required: the elements
of a `Vec<F>` move when the vector reallocates or removes an element, so they cannot be pinned in
place without `unsafe`.

```rust
fn run_round_robin<F: Future<Output = u32>>(tasks: Vec<F>) -> Vec<(usize, u32)> {
    let mut cx = Context::from_waker(Waker::noop());
    let mut unfinished: Vec<(usize, Pin<Box<F>>)> =
        tasks.into_iter().map(Box::pin).enumerate().collect();
    let mut finished = Vec::new();
    while !unfinished.is_empty() {
        unfinished.retain_mut(|(index, task)| match task.as_mut().poll(&mut cx) {
            Poll::Ready(output) => {
                finished.push((*index, output));
                false
            }
            Poll::Pending => true,
        });
    }
    finished
}

pub fn squares_round_robin(n: u32) -> Vec<u32> {
    let tasks: Vec<_> = (0..n)
        .map(|i| async move {
            let mut square = 0;
            for _ in 0..i {
                square += i;
                yield_now().await;
            }
            square
        })
        .collect();
    let mut squares = vec![0; n as usize];
    for (index, output) in run_round_robin(tasks) {
        squares[index] = output;
    }
    squares
}

pub fn finish_order(delays: &[u32]) -> Vec<usize> {
    let tasks: Vec<_> = delays
        .iter()
        .map(|&delay| async move {
            for _ in 0..delay {
                yield_now().await;
            }
            delay
        })
        .collect();
    run_round_robin(tasks).into_iter().map(|(index, _)| index).collect()
}
```

A boxed future can also be pinned later, because the box owns it: `Box::into_pin` (or
`Pin::from`) turns a `Box<T>` into a `Pin<Box<T>>` without moving `T`, and it is safe because the
unpinned box is consumed. Then the `Task` alias and the `Box::new` calls stay as they are, and only
the executor changes:

```rust
fn run_round_robin(tasks: Vec<Task>) -> Vec<(usize, u32)> {
    let mut cx = Context::from_waker(Waker::noop());
    let mut unfinished: Vec<(usize, Pin<Task>)> =
        tasks.into_iter().map(Box::into_pin).enumerate().collect();
    let mut finished = Vec::new();
    while !unfinished.is_empty() {
        unfinished.retain_mut(|(index, task)| match task.as_mut().poll(&mut cx) {
            Poll::Ready(output) => {
                finished.push((*index, output));
                false
            }
            Poll::Pending => true,
        });
    }
    finished
}
```

Once the tasks are `Pin<Box<_>>`, `Pin::new(task).poll(&mut cx)` also compiles: the
`Pin<Box<dyn Future>>` is `Unpin`, and it is a future itself, because `Pin<P>` implements `Future`
whenever `P` is a mutable pointer to a future.
