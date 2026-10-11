This code compiles, and it inserts every value in the right place. But it contains **undefined
behavior (UB)**, and Miri detects it.

`insert_sorted` inserts `x` into the vector `v`, which is sorted by the comparison function `less`,
so that `v` stays sorted.

```rust
pub fn insert_sorted<T, F: FnMut(&T, &T) -> bool>(v: &mut Vec<T>, x: T, less: F)
```

- `less(a, b)` returns `true` when `a` must come before `b`. It is a strict order like `<`: two
  values are equal when neither is less than the other. `v` is sorted: no element of `v` is less
  than an element before it.
- `x` is inserted just before the first element `e` of `v` for which `less(&x, e)` is `true`, or at
  the end if there is no such element. So `x` goes after all the elements that are equal to it.
- How many times `less` is called, and with which pairs of values, is up to you.
- `less` may panic. `insert_sorted` must not catch the panic: it propagates to the caller. After
  that, `v` must still be a valid vector that holds each element it had before the call exactly
  once, in the same order. `x` is then either in `v` (once, at any position) or already dropped.
- With or without a panic, every value is dropped exactly once in the end: no value is dropped
  twice, and none is leaked.

Remove the UB so that it works correctly. (You may keep using `unsafe` if you like.)

### Examples

- Inserting `"c"` into `["b", "d"]` with `|a, b| a < b` gives `["b", "c", "d"]`.
- Inserting `(2, 'x')` into `[(1, 'a'), (2, 'b'), (3, 'c')]` with `|a, b| a.0 < b.0` gives
  `[(1, 'a'), (2, 'b'), (2, 'x'), (3, 'c')]`.
- Inserting `"c"` into `["b", "d"]` (as `String`s) with a `less` that panics on its second call:
  the panic reaches the caller, and `v` then holds `"b"` and `"d"` in this order, and possibly
  `"c"` once at any position.
