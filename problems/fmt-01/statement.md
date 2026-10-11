`Money` is an amount of money stored as a number of cents (it is already in the editor). Implement
`Display` for it so that it prints like a price tag.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub cents: i64,
}

impl fmt::Display for Money { /* to implement */ }
```

- The text is a `$`, the whole dollars, a `.` and exactly two digits of cents: `123456` cents is
  `$1,234.56`, `5` is `$0.05` and `0` is `$0.00`.
- The dollars are split into groups of three digits **from the right** with `,`. There is always at
  least one digit before the `.`, and no leading zeros.
- A negative amount starts with `-` before the `$`: `-5` is `-$0.05`. Every `i64` value must work,
  including `i64::MIN` and `i64::MAX`.
- The text must follow the width, fill character and alignment of the format spec, the same way a
  string does:
  - With a width larger than the text, the fill character (a space by default) is added until the
    result has exactly that many characters. Without a width, or with a width that is not larger
    than the text, nothing is added.
  - `<` puts the text on the left, `>` on the right, and `^` in the center. With `^`, when the
    padding cannot be split evenly, the extra fill character goes on the right. Without an
    alignment, the text goes on the left.
- Precision and flags such as `+`, `#` or `0` are not used.

### Examples

| Code | Result |
| --- | --- |
| `Money { cents: 123456 }.to_string()` | `"$1,234.56"` |
| `format!("{}", Money { cents: -5 })` | `"-$0.05"` |
| `format!("{}", Money { cents: -100000000 })` | `"-$1,000,000.00"` |
| `format!("{:>10}", Money { cents: 5 })` | `"     $0.05"` |
| `format!("{:>8}", Money { cents: -100 })` | `"  -$1.00"` |
| `format!("{:*<9}", Money { cents: 5 })` | `"$0.05****"` |
| `format!("{:^11}", Money { cents: 5 })` | `"   $0.05   "` |
