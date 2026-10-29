The code below does not compile. Fix it.

The function `select` picks elements of a slice and returns them as an `impl Iterator`. The `Keep`
enum is already in the editor.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keep {
    Evens,
    Odds,
    All,
}

pub fn select(v: &[i32], keep: Keep) -> impl Iterator<Item = i32> + '_
```

- `select(v, keep)` yields elements of `v` in their original order: the even ones for
  `Keep::Evens`, the odd ones for `Keep::Odds`, and all of them for `Keep::All`.
- Negative numbers count too: `-4` and `0` are even, and `-3` is odd.

Do not change `Keep` or the signature of `select`.

### Examples

- `select(&[1, -3, 4], Keep::Odds)` yields `1`, then `-3`.
