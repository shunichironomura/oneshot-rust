The judge builds your code with overflow checks, as debug builds do: an arithmetic operation whose
result does not fit in its type panics ("attempt to add with overflow") instead of silently
wrapping around as in a release build. The obvious `xs.iter().sum::<u8>()` therefore panics as
soon as the sum passes 255.

`checked_add` returns `None` on overflow, and `try_fold` stops at the first `None` that its closure
returns, which makes the function a one-liner.

A near miss still panics: `xs.iter().map(|&x| Some(x)).sum::<Option<u8>>()` returns `None` only
when an item is `None`; the additions themselves are ordinary ones and panic on overflow.

### Alternatives

`sum_checked` can be a loop in which `?` returns `None` as soon as `checked_add` does.

```rust
pub fn sum_checked(xs: &[u8]) -> Option<u8> {
    let mut total: u8 = 0;
    for &x in xs {
        total = total.checked_add(x)?;
    }
    Some(total)
}
```

Summing in a wider type and converting with `u8::try_from(total).ok()` also works, but only if the
wider type is wide enough: the bytes of a long slice overflow even a `u16`.
