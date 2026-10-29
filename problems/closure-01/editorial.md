A closure that changes a captured variable implements `FnMut`, which is what the counter needs. The
closure outlives the call to `make_counter`, so it must **own** `step` and `current`: without `move`
it would only borrow two local variables that are gone when the function returns, and the compiler
rejects it.

`impl FnMut() -> u32` in return position lets you return a closure, whose type has no name, without
boxing it.

### Alternatives

Count the calls instead of the running total, or let an iterator keep the state. The iterator is
captured by `move` too, and `next()` needs `&mut`, so the closure is still `FnMut`.

```rust
pub fn make_counter(step: u32) -> impl FnMut() -> u32 {
    let mut multiples = (1..).map(move |k| k * step);
    move || multiples.next().unwrap()
}
```

Returning `Box<dyn FnMut() -> u32>` also works, but it does not match the given signature and costs
an allocation and a dynamic call.
