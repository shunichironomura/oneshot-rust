The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Each of the five functions uses a `match` to sort its input into one of a few cases.

```rust
pub fn digits_label(n: u8) -> &'static str;
pub fn sign_word(n: i32) -> &'static str;
pub fn quadrant(x: i32, y: i32) -> &'static str;
pub fn grade(score: u32) -> char;
pub fn parse_answer(s: &str) -> Option<bool>;
```

- `digits_label` returns how many decimal digits `n` has: `"one digit"` for 0 to 9, `"two digits"`
  for 10 to 99, and `"three digits"` for 100 to 255.
- `sign_word` returns `"negative"`, `"zero"` or `"positive"` according to the sign of `n`.
- `quadrant` returns where the point `(x, y)` lies: `"I"` if `x > 0` and `y > 0`, `"II"` if `x < 0`
  and `y > 0`, `"III"` if `x < 0` and `y < 0`, and `"IV"` if `x > 0` and `y < 0`. A point on an axis
  is in no quadrant: `(0, 0)` is `"origin"`, any other point with `x == 0` is `"y-axis"`, and any
  other point with `y == 0` is `"x-axis"`.
- `grade` turns a score into a letter: `'A'` for 90 or more, `'B'` for 80 to 89, `'C'` for 70 to 79,
  `'D'` for 60 to 69, and `'F'` below 60. Extra credit can push a score above 100; such a score is
  still an `'A'`.
- `parse_answer` reads a yes/no answer: `"yes"` and `"y"` give `Some(true)`, `"no"` and `"n"` give
  `Some(false)`, and any other string gives `None`. The comparison is exact, with no case folding
  and no trimming, so `"Yes"` and `" y"` give `None`.

Do not change the signatures.

### Examples

| Call | Result |
| --- | --- |
| `digits_label(42)` | `"two digits"` |
| `sign_word(-7)` | `"negative"` |
| `quadrant(-2, 5)` | `"II"` |
| `quadrant(0, -3)` | `"y-axis"` |
| `grade(85)` | `'B'` |
| `parse_answer("y")` | `Some(true)` |
| `parse_answer("maybe")` | `None` |
