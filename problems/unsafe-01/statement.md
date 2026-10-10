This code compiles, and it may even seem to work when you run it natively. But it contains
**undefined behavior (UB)**, and Miri detects it.

`push_and_sum` appends `extra` to `v` and returns the sum of all elements after the push plus the
first element **before** the push (`v` is never empty).

```rust
pub fn push_and_sum(v: &mut Vec<i32>, extra: i32) -> i32
```

Remove the UB so that it works correctly. (You may keep using `unsafe` if you like.)

### Example

With `v = [1, 2, 3]` and `extra = 4`, `v` becomes `[1, 2, 3, 4]` and the result is `10 + 1 = 11`.
