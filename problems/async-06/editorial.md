The future of an `async fn` is a value that stores everything alive across an `.await`, including
the future being awaited. Here that is the future of `depth` itself, so the type would contain
itself and be infinitely large (E0733). `Box::pin(depth(c))` moves the inner future to the heap, so
the outer future only holds a pointer. Since Rust 1.77, this is all a recursive `async fn` needs.

- `Box::new` is not enough: a `Box<F>` can only be awaited when `F: Unpin`, which the future of an
  `async fn` is not.
- `std::pin::pin!` pins on the stack, so the future stays infinitely large.

### Alternatives

Before Rust 1.77, a recursive async function had to return a boxed future explicitly. Keep the
public `async fn` and let it call a private helper that does. The `+ '_` is needed because a boxed
`dyn Future` defaults to `'static`, while this future borrows `t`.

```rust
use std::pin::Pin;

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
