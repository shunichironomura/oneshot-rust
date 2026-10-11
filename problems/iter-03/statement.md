Implement four small functions. Each one fits in a few lines, but every detail of the rules below
matters.

```rust
pub fn last_long(words: &[&str], min: usize) -> Option<usize>
pub fn split_header<'a>(lines: &[&'a str]) -> (Vec<&'a str>, Vec<&'a str>)
pub fn running_balance(start: i64, txns: &[i64]) -> Vec<i64>
pub fn powers_of(base: u64) -> Vec<u64>
```

- `last_long` returns the position of the **last** word that has at least `min` characters. The
  position is counted from the start of `words`: the first word is at position `0`. Characters are
  counted as `char`s (Unicode scalar values), not bytes: `"é"` and `"日"` are one character each.
  If no word is long enough (or `words` is empty), it returns `None`.
- `split_header` splits `lines` into a header and a body. The header is the longest run of lines at
  the start of `lines` that begin with `#` (the first character of the line is `#`; leading spaces
  are not skipped). The body is all the remaining lines, starting with the first line that does not
  begin with `#`; lines after it stay in the body even if they begin with `#`. Both keep the original
  order, and either may be empty.
- `running_balance` starts with the balance `start` and applies the transactions `txns` in order,
  adding each one to the balance. It returns the balance after each applied transaction. A
  transaction that would make the balance negative is not applied, and processing stops there: neither
  it nor any later transaction is applied or appears in the result. A balance of exactly `0` is
  fine. `start` is not negative, and no sum overflows `i64`.
- `powers_of` returns the powers of `base` in increasing order, `1, base, base², ...`, up to the
  largest power that fits in a `u64`. `base` is at least `2`.

### Examples

- `last_long(&["tea", "coffee", "milk", "juice"], 5)` returns `Some(3)`.
- `last_long(&["abc", "日本"], 3)` returns `Some(0)`.
- `split_header(&["# a", "# b", "body", "# x"])` returns
  `(vec!["# a", "# b"], vec!["body", "# x"])`.
- `running_balance(100, &[-30, 50, -200, 10])` returns `vec![70, 120]`.
- `powers_of(10)` has 20 elements, from `1` to `10_000_000_000_000_000_000` (10¹⁹).
- The last element of `powers_of(2)` is `1 << 63` (2⁶³).
