`Option` orders `None` **before** every `Some`. Here `None` means "no pre-release", which has to
come **after** every pre-release of the same version, so the obvious shortcuts give the wrong
order: `#[derive(PartialOrd, Ord)]`, `self.pre.cmp(&other.pre)`, and a tuple comparison ending in
`&self.pre` all put `1.0.0` before `1.0.0-beta`. Compare the numbers first, then `match` on both
`Option`s. `Ordering::then` and `then_with` chain the comparisons: the next one only counts when
everything before it was `Equal`, and `then_with` takes a closure, so the `match` only runs in that
case.

`<`, `>` and `partial_cmp` call `PartialOrd`. `sort`, `max`, `min` and `BTreeSet` require `Ord`,
but inside they may call the methods of either trait (`sort`, for example, compares with `<`), so
the two must agree. Writing `partial_cmp` as `Some(self.cmp(other))` guarantees that. The order
must also agree with the derived `PartialEq`, `Eq` and `Hash`: `cmp` must return `Equal` exactly
when all four fields are equal. It does, because it looks at every field and compares the labels
exactly. Comparing them while ignoring case, for example, would make `1.2.0-RC1` and `1.2.0-rc1`
"equal" for a `BTreeSet` but not for a `HashSet`.

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
