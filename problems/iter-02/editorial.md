The six closures look alike, but they do not all receive the same type. Count the `&`s:

- `v.iter()` yields `&i32`; `words.iter()` yields `&&str` or `&String`.
- `find` and `filter` still need the item after asking the closure (to return it or to pass it on),
  so the closure only gets a **reference to the item**: `&&i32`.
- `position`, `any` and `map` are done with the item after the call, so the closure gets the item
  itself: `&i32`.

What each body does with its argument then decides whether it compiles:

- Operators do not look through references. `x < 0` compares a `&&i32` with an integer (E0308), and
  `x % 2` is not defined for `&&i32` (E0369; it is defined for `i32` and `&i32`, but not for two
  layers). Writing `|&&x|` or `**x` gets down to the `i32`.
- `find` returns the item it found, so the result is an `Option<&i32>`, and `filter` passes on
  `&i32` items, which cannot be collected into a `Vec<i32>` (E0277). `copied()` turns `&i32` items
  into `i32` (`i32` is `Copy`). Calling it right after `iter()` fixes both problems at once: the
  closures get one layer less and the results are plain `i32`s. The closures then get a `&i32`:
  `%` accepts it, but `<` wants the same type on both sides, hence `|&x|` in `first_negative`.
- In `position_of_max`, `x == max` compares two `&i32`, which is fine.
- Method calls, unlike operators, dereference automatically. In `count_long` the closure gets a
  `&&&str`, three layers, and `w.len()` still finds `str::len`. So `position_of_max` and
  `count_long` were already correct.
- `slice::contains` takes a `&T`, here a `&String`, and a `&str` is not one (E0308). `==` is more
  flexible: `String` can be compared with `str`, so `s == w` works for `s: &String` and `w: &str`.
  Comparing with `==` also matches whole words only, unlike `s.contains(w)`, which looks for a part.
- The pattern `|&w|` takes the `String` out of the reference, which would move it (E0507). The same
  pattern works for `i32` only because `i32` is `Copy`. Take `|w|` and call the method on it.

### Alternatives

Keep the template's shape and strip the layers in the closures with patterns or `*`, then copy the
result. A method path such as `String::len` can replace a closure that only calls a method.

```rust
pub fn first_negative(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x < 0).copied()
}

pub fn evens(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|x| **x % 2 == 0).copied().collect()
}

pub fn contains_word(words: &[String], w: &str) -> bool {
    words.iter().any(|s| s.as_str() == w)
}

pub fn total_len(words: &[String]) -> usize {
    words.iter().map(String::len).sum()
}
```

`words.contains(&w.to_string())` also works, but it builds a new `String` just for the comparison.

Plain `for` loops avoid the question of layers. `for &x in v` copies each `i32` out of the slice.

```rust
pub fn first_negative(v: &[i32]) -> Option<i32> {
    for &x in v {
        if x < 0 {
            return Some(x);
        }
    }
    None
}

pub fn evens(v: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &x in v {
        if x % 2 == 0 {
            result.push(x);
        }
    }
    result
}
```
