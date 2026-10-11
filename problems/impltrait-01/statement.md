The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Each of the three functions returns an `impl Trait`. The `Keep` enum is already in the editor.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keep {
    Evens,
    Odds,
    All,
}

pub fn select(v: &[i32], keep: Keep) -> impl Iterator<Item = i32> + '_
pub fn describe(n: i32) -> impl Display
pub fn op_for(symbol: char, scale: i32) -> impl Fn(i32, i32) -> i32
```

- `select(v, keep)` yields elements of `v` in their original order: the even ones for
  `Keep::Evens`, the odd ones for `Keep::Odds`, and all of them for `Keep::All`. Negative numbers
  count too: `-4` and `0` are even, and `-3` is odd.
- `describe(n)` is displayed (with `{}` or `to_string()`) as the word `minus`, one space and the
  absolute value of `n` when `n` is negative, and as the number itself otherwise. This must work for
  every `i32`, including `i32::MIN`.
- `op_for(symbol, scale)` returns a function `f`, and `f(a, b)` is:
  - `(a + b) * scale` for `'+'`,
  - `(a - b) * scale` for `'-'`,
  - `a * b * scale` for `'*'`,
  - `a * scale` for any other symbol (`b` is ignored).

  The harness may call each returned function several times. The values are small, so nothing
  overflows.

Do not change `Keep` or the signatures of `select` and `describe`. In `op_for`, only the return type
may change, as long as the result can still be called as `f(a, b)`.

### Examples

- `select(&[1, -3, 4], Keep::Odds)` yields `1`, then `-3`.
- `describe(-5).to_string()` is `"minus 5"`, and `describe(12).to_string()` is `"12"`.
- `op_for('-', 2)(7, 3)` is `8`.
