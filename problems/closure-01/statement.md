Implement two functions that build new functions.

```rust
pub fn make_counter(step: u32) -> impl FnMut() -> u32
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C
```

- `make_counter(step)` returns a counter. Each call returns the next multiple of `step`: `step`,
  `2 * step`, `3 * step`, and so on. Every counter keeps its own count.
- `compose(f, g)` returns a function that applies `f` first and then `g` to the result.

### Examples

```rust
let mut c = make_counter(3);
// c() returns 3, then 6, then 9

let h = compose(|x: i32| x + 1, |x: i32| x * 10);
// h(4) returns 50
```
