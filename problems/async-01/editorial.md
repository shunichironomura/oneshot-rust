Each function breaks a different rule of async Rust.

**`greet`**: an `async` block captures variables like a closure does, by reference when that is
enough. Formatting only needs `&name`, so the returned future borrows `name`, a local that is
dropped when `greet` returns (E0597). `async move` moves `name` into the future instead. An
`async fn` never has this problem, because it always moves its parameters into its future. Writing
`name.clone()` inside the block does not help: the clone runs only when the future is polled, so the
block still borrows `name`.

**`sum_all`**: `.await` is only allowed directly in the body of an `async fn` or an `async` block.
The closure `|f| f.await` is an ordinary closure, even though it is written inside an async function
(E0728): `map` and `sum` call it synchronously, so there is nowhere to suspend. Making it an async
closure (`async |f| f.await`) does not help either: `map` then yields futures, which `sum` cannot
add up. A `for` loop has no closure, so its body is part of the async function. It also awaits the
futures one at a time, in order. Popping them with `while let Some(f) = futs.pop()` (after making
`futs` mutable) compiles too, but awaits them in reverse order.

**`depth`**: the future of an `async fn` is a value that stores everything alive across an
`.await`, including the future being awaited. Here that is the future of `depth` itself, so the type
would contain itself and be infinitely large (E0733). `Box::pin(depth(c))` moves the inner future to
the heap, so the outer future only holds a pointer. Since Rust 1.77, this is all a recursive
`async fn` needs. `Box::new` is not enough: a `Box<F>` can only be awaited when `F: Unpin`, which
the future of an `async fn` is not. `std::pin::pin!` pins on the stack, so the future stays
infinitely large.

### Alternatives

`greet` can format the string first and return a future that is already finished. The output is the
same, but the formatting now happens when `greet` is called rather than on the first poll.

```rust
pub fn greet(name: String) -> impl Future<Output = String> {
    std::future::ready(format!("Hello, {name}!"))
}
```

Before Rust 1.77, a recursive async function had to return a boxed future explicitly. Keep the
public `async fn` and let it call a private helper that does. The `+ '_` is needed because a boxed
`dyn Future` defaults to `'static`, while this future borrows `t`.

```rust
pub async fn depth(t: &Tree) -> usize {
    depth_boxed(t).await
}

fn depth_boxed(t: &Tree) -> Pin<Box<dyn Future<Output = usize> + '_>> {
    Box::pin(async move {
        let mut d = 0;
        for c in &t.children {
            d = d.max(depth_boxed(c).await);
        }
        d + 1
    })
}
```
