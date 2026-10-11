`Option` orders `None` **before** every `Some`. Here `None` means "no pre-release", which has to
come **after** every pre-release of the same version, so the obvious shortcuts give the wrong
order: `#[derive(PartialOrd, Ord)]`, `self.pre.cmp(&other.pre)`, and a tuple comparison ending in
`&self.pre` all put `1.0.0` before `1.0.0-beta`. Compare the numbers first, then `match` on both
`Option`s. `Ordering::then` and `then_with` chain the comparisons: the next one only counts when
everything before it was `Equal`, and `then_with` takes a closure, so the `match` only runs in that
case.

`<`, `>` and `partial_cmp` use `PartialOrd`, while `sort`, `max`, `min` and `BTreeSet` use `Ord`.
Writing `partial_cmp` as `Some(self.cmp(other))` guarantees that the two never disagree. The order
must also agree with the derived `PartialEq`, `Eq` and `Hash`: `cmp` must return `Equal` exactly
when all four fields are equal. It does, because it looks at every field and compares the labels
exactly. Comparing them while ignoring case, for example, would make `1.2.0-RC1` and `1.2.0-rc1`
"equal" for a `BTreeSet` but not for a `HashSet`.

For parsing, `u64::from_str` (what `parse::<u64>()` calls) is not a digits-only check: it accepts a
leading `+`, so `"+1"` and `"1.+2"` would slip through. Check that a part consists of ASCII digits
first, and then let `parse` report an overflow as an error. Likewise, `char::is_alphanumeric`
accepts every Unicode letter and digit, such as `β`, `é` or `٣`; the ASCII-only checks are
`is_ascii_alphanumeric` and `is_ascii_digit`. Split off the label at the first `-` **before**
splitting at `.`, because the label may contain dots (`3-rc.1`). Collecting into
`Result<Vec<_>, _>` stops at the first bad part, and a slice pattern then accepts one, two or three
numbers and nothing else.

`latest_stable` is `filter` followed by `max`. `&Version` implements `Ord` through `Version`, so
`max` directly gives the `Option<&Version>` to return.

### Alternatives

A key tuple works if `pre.is_none()` comes before the label: `false < true`, so a version without a
pre-release is greater, and the label only matters when both have one (two `None`s are equal).

```rust
impl Version {
    fn key(&self) -> (u64, u64, u64, bool, Option<&str>) {
        (self.major, self.minor, self.patch, self.pre.is_none(), self.pre.as_deref())
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}
```

`std::cmp::Reverse` can flip the order of `Option` too. `Reverse(pre.map(Reverse))` puts `None`
last, and the labels, reversed twice, keep their order.

```rust
use std::cmp::Reverse;

fn key(v: &Version) -> (u64, u64, u64, Reverse<Option<Reverse<&str>>>) {
    (v.major, v.minor, v.patch, Reverse(v.pre.as_deref().map(Reverse)))
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        key(self).cmp(&key(other))
    }
}
```

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
