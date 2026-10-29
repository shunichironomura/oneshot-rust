The error comes from **variance**: the rules that say when a type with one lifetime may be used as
the same type with another lifetime.

The caller is fine and the helper is wrong. `add_word` takes `&'a mut Vec<&'a str>`, which ties the
length of the mutable borrow to the lifetime of the strings in the vector. `&mut T` is *invariant*
in `T`, so that lifetime cannot be shortened to make the borrow short, and it must cover every later
use of `seen`. So the first `&mut seen` stays active until `seen` is used for the last time: the next
iteration borrows it again (E0499), and `seen.iter()` reads it while it is still mutably borrowed
(E0502). The errors point at the loop, but the cause is the signature of the helper.

Give the borrow its own lifetime: `fn add_word<'a>(seen: &mut Vec<&'a str>, word: &'a str)`. The
borrow of `seen` then lasts only for the call, while the words keep the lifetime of `text`. A
`&'a mut Thing<'a>`, including `&'a mut self` in an `impl<'a> Thing<'a>`, is almost never what you
want.

### Alternatives

The elided lifetime of the borrow can be written out. The two lifetimes are independent.

```rust
fn add_word<'s, 'a>(seen: &'s mut Vec<&'a str>, word: &'a str)
```

You can also remember the words seen so far in a `HashSet<&str>`. `insert` returns `false` for a word
that is already in the set, so it fits `filter` directly, with no helper and no lifetime annotations
at all.

```rust
use std::collections::HashSet;

pub fn unique_upper(text: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    text.split_whitespace()
        .filter(|word| seen.insert(*word))
        .map(str::to_uppercase)
        .collect()
}
```
