The three closures look alike, but they do not all receive the same type. Count the `&`s:

- `v.iter()` yields `&i32`; `words.iter()` yields `&&str`.
- `find` and `filter` still need the item after asking the closure (to return it or to pass it on),
  so the closure only gets a **reference to the item**: `&&i32` for `v.iter()`, `&&&str` for
  `words.iter()`.
- `position` is done with the item after the call, so the closure gets the item itself: `&i32` for
  `v.iter()`.

What each body does with its argument then decides whether it compiles:

- Operators do not dereference automatically: they only work for the types they are implemented
  for. `x < 0` compares a `&&i32` with an integer (E0308). Writing `|&&x|` or `**x` gets down to
  the `i32`.
- `find` returns the item it found, so the result is an `Option<&i32>`, not an `Option<i32>`.
  `copied()` turns `&i32` items into `i32` (`i32` is `Copy`). Calling it right after `iter()` fixes
  both problems at once: the closure gets one layer less and the result is a plain `i32`. The
  closure then gets a `&i32`, and `<` wants the same type on both sides, hence `|&x|`.
- In `position_of_max`, `x == max` compares two `&i32`, which is fine.
- Method calls, unlike operators, dereference automatically. In `count_long` the closure gets a
  `&&&str`, three layers, and `w.len()` still finds `str::len`. So `position_of_max` and
  `count_long` were already correct.

### Alternatives

Keep the template's shape and strip the layers in the closure with a pattern, then copy the result.

```rust
pub fn first_negative(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x < 0).copied()
}
```

A plain `for` loop avoids the question of layers. `for &x in v` copies each `i32` out of the slice.

```rust
pub fn first_negative(v: &[i32]) -> Option<i32> {
    for &x in v {
        if x < 0 {
            return Some(x);
        }
    }
    None
}
```
