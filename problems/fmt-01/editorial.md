The trap is the usual way of writing `Display`. `write!(f, "{sign}${dollars}.{cents:02}")` prints
the right text but drops the padding. The width, fill and alignment the caller asked for (`{:>10}`)
are stored in `f`, but `write!` formats its pieces only with the specs written in its own format
string, so nothing reads them. `{:>10}` would print just `$0.05`.

`Formatter::pad` is what `str` itself uses: it writes a string after applying the width, fill and
alignment of `f` (and the precision, as a maximum length). So build the whole text first, sign
included, and hand it to `f.pad(&text)`. Padding only part of it does not work: writing the `-`
first and then padding the rest would put the fill between `-` and `$`.

The number itself:

- `self.cents.abs()` overflows for `i64::MIN`, because `i64::MAX` is one smaller than its magnitude
  (a panic in debug builds). `unsigned_abs()` returns a `u64` and works for every value.
- Take the sign from `self.cents`, not from the dollars: `-5 / 100` is `0`, which has no sign.
- Group the digits from the right. Walking from the left, a comma goes before the digit at index `i`
  (except the first one) when `len - i`, the number of digits from there to the end, is a multiple
  of three.
- `{:02}` prints the cents with a leading zero.

### Alternatives

Group with arithmetic: peel off three digits at a time with `% 1000` and print each with `{:03}`, so
that a group like `005` keeps its zeros; only the leftmost group is printed without padding.
Instead of `f.pad`, `fmt::Display::fmt(&text, f)` also works: it calls the `Display` of `String`
with the same formatter, which pads the text.

```rust
fn group_thousands(mut n: u64) -> String {
    let mut groups = Vec::new();
    while n >= 1000 {
        groups.push(format!("{:03}", n % 1000));
        n /= 1000;
    }
    groups.push(n.to_string());
    groups.reverse();
    groups.join(",")
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abs = self.cents.unsigned_abs();
        let sign = if self.cents < 0 { "-" } else { "" };
        let text = format!("{sign}${}.{:02}", group_thousands(abs / 100), abs % 100);
        fmt::Display::fmt(&text, f)
    }
}
```

You can also pad by hand with `f.width()`, `f.fill()` and `f.align()`, in place of `f.pad(&text)`.
Use `saturating_sub`, since a width smaller than the text would make `width - len` underflow.

```rust
let len = text.chars().count();
let padding = f.width().unwrap_or(0).saturating_sub(len);
let (left, right) = match f.align() {
    Some(fmt::Alignment::Right) => (padding, 0),
    Some(fmt::Alignment::Center) => (padding / 2, padding - padding / 2),
    Some(fmt::Alignment::Left) | None => (0, padding),
};
let fill = f.fill().to_string();
write!(f, "{}{text}{}", fill.repeat(left), fill.repeat(right))
```
