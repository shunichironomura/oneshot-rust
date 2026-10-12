`u64::from_str` (what `parse::<u64>()` calls) is not a digits-only check: it accepts a leading `+`,
so `"+1"` and `"1.+2"` would slip through. Check that a part consists of ASCII digits first, and
then let `parse` report an overflow as an error. Likewise, `char::is_alphanumeric` accepts every
Unicode letter and digit, such as `β`, `é` or `٣`; the ASCII-only checks are
`is_ascii_alphanumeric` and `is_ascii_digit`.

Split off the label at the first `-` **before** splitting at `.`, because the label may contain
dots (`3-rc.1`). Collecting into `Result<Vec<_>, _>` stops at the first bad part, and a slice
pattern then accepts one, two or three numbers and nothing else.

### Alternatives

Without `parse`, add up the digits with `checked_mul` and `checked_add`, which turn an overflow into
`None` instead of a panic. `to_digit(10)` only accepts `0` to `9`, so a sign is rejected as well.

```rust
fn parse_number(s: &str) -> Result<u64, ParseVersionError> {
    if s.is_empty() {
        return Err(ParseVersionError);
    }
    s.chars()
        .try_fold(0u64, |n, c| {
            let digit = c.to_digit(10)?;
            n.checked_mul(10)?.checked_add(u64::from(digit))
        })
        .ok_or(ParseVersionError)
}
```
