Implement a function that builds a counter and returns it as a closure.

```rust
pub fn make_counter(step: u32) -> impl FnMut() -> u32
```

- `make_counter(step)` returns a counter. Each call returns the next multiple of `step`: `step`,
  `2 * step`, `3 * step`, and so on.
- Every counter keeps its own count.

### Examples

```rust
let mut c = make_counter(3);
// c() returns 3, then 6, then 9
```
