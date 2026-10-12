The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Each of the three functions takes its input apart with a pattern.

```rust
pub fn first_even(v: &[i32]) -> Option<i32>;
pub fn title_or_default(title: Option<String>) -> (bool, String);
pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)>;
```

- `first_even` returns the first element of `v` that is even, or `None` if there is none. Zero and
  negative numbers count too: `0` and `-8` are even, `-3` is odd.
- `title_or_default` tells whether a title was given, and which title to show. `Some(t)` gives
  `(true, t)` with `t` unchanged, even if it is empty. `None` gives `(false, "Untitled")`.
- `filled_slots` returns the position and the value of every `Some` in `slots`, in order. Positions
  start at 0 and count the `None`s too. `Some(0)` is a filled slot holding `0`.

Do not change the signatures.

### Examples

| Call | Result |
| --- | --- |
| `first_even(&[1, 4, 6])` | `Some(4)` |
| `title_or_default(Some("Notes".to_string()))` | `(true, "Notes")` |
| `title_or_default(None)` | `(false, "Untitled")` |
| `filled_slots(&[None, Some(4), Some(7)])` | `[(1, 4), (2, 7)]` |
