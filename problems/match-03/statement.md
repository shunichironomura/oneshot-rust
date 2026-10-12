The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Each of the three functions uses a `match` to sort its input into one of a few cases.

```rust
pub fn digits_label(n: u8) -> &'static str;
pub fn sign_word(n: i32) -> &'static str;
pub fn quadrant(x: i32, y: i32) -> &'static str;
```

- `digits_label` returns how many decimal digits `n` has: `"one digit"` for 0 to 9, `"two digits"`
  for 10 to 99, and `"three digits"` for 100 to 255.
- `sign_word` returns `"negative"`, `"zero"` or `"positive"` according to the sign of `n`.
- `quadrant` returns where the point `(x, y)` lies: `"I"` if `x > 0` and `y > 0`, `"II"` if `x < 0`
  and `y > 0`, `"III"` if `x < 0` and `y < 0`, and `"IV"` if `x > 0` and `y < 0`. A point on an axis
  is in no quadrant: `(0, 0)` is `"origin"`, any other point with `x == 0` is `"y-axis"`, and any
  other point with `y == 0` is `"x-axis"`.

Do not change the signatures.

### Examples

| Call | Result |
| --- | --- |
| `digits_label(42)` | `"two digits"` |
| `sign_word(-7)` | `"negative"` |
| `quadrant(-2, 5)` | `"II"` |
| `quadrant(0, -3)` | `"y-axis"` |
