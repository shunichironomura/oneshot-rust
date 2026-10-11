This code compiles, and it even works for vectors of `i32`. But it contains **undefined behavior
(UB)**, and Miri detects it.

`build` makes a vector whose element at index `i` is `f(i)`. It does not compute the elements from
the first to the last, though: it calls `f` for the indices in the order given by `order` (for
example, the most urgent elements first).

```rust
pub fn build<T>(order: &[usize], f: impl FnMut(usize) -> T) -> Vec<T>
```

- `order` is a permutation of the indices `0, 1, ..., n - 1`, where `n` is `order.len()`: each of
  them appears exactly once. (Other inputs are not tested.)
- `build` calls `f(order[0])`, `f(order[1])`, ..., `f(order[n - 1])`, in this order, and calls `f`
  at no other time. So `f` is called exactly once per index.
- It returns the vector `[f(0), f(1), ..., f(n - 1)]`: each value that `f` returns goes to the index
  that `f` was called with. For an empty `order`, it returns an empty vector without calling `f`.
- `f` may panic. `build` must not catch the panic: it propagates to the caller. By the time the
  panic reaches the caller, every value that `f` returned before the panic has been dropped exactly
  once.
- With or without a panic, every value is dropped exactly once in the end: no value is dropped
  twice, and none is leaked.

Remove the UB so that it works correctly. (You may keep using `unsafe` if you like.)

### Examples

- `build(&[2, 0, 1], |i| i.to_string())` calls `f(2)`, `f(0)` and `f(1)`, in this order, and
  returns `["0", "1", "2"]`.
- With `order = [3, 1, 0, 2]` and an `f` that panics when it is called with `0` (its third call),
  the panic reaches the caller, and by then the values that `f(3)` and `f(1)` returned have each
  been dropped exactly once.
