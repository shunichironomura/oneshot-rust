All three errors come from **variance**: the rules that say when a type with one lifetime may be used
as the same type with another lifetime.

In `merge`, the helper `push_unique` is fine; the local variable is the problem. `all` is declared as
`Vec<&'static str>`, so `&mut all` is a `&mut Vec<&'static str>`. `&mut T` is *invariant* in `T`: it
cannot be passed as a `&mut Vec<&'a str>` with a shorter `'a`, because the callee could then push a
short-lived string into a vector that promises to hold only `'static` ones. So `'a` is forced to be
`'static`, and `word`, which borrows `extra`, does not live that long. No signature of `push_unique`
can fix this, because a `Vec<&'static str>` simply cannot hold `word`.

Write `Vec<&str>` instead, or leave the annotation out. Shared references and `Vec` are *covariant*:
a `&'static str` can be used as a `&'a str` for any shorter `'a`, so the `Vec<&'static str>` from
`defaults.to_vec()` can become a `Vec<&'a str>` whose lifetime fits both the defaults and the words of
`extra`. The `'static` in the type of `defaults` does not have to be carried into the local variable.

In `unique_upper`, the caller is fine and the helper is wrong. `add_word` takes
`&'a mut Vec<&'a str>`, which ties the length of the mutable borrow to the lifetime of the strings in
the vector. Because of invariance, that lifetime cannot be shortened to make the borrow short, and it
must cover every later use of `seen`. So the first `&mut seen` stays active until `seen` is used for
the last time: the next iteration borrows it again (E0499), and `seen.iter()` reads it while it is
still mutably borrowed (E0502). Give the borrow its own lifetime, as `push_unique` already does:
`fn add_word<'a>(seen: &mut Vec<&'a str>, word: &'a str)`. A `&'a mut Thing<'a>`, including
`&'a mut self` in an `impl<'a> Thing<'a>`, is almost never what you want.

Making the words `'static` by leaking them (`Box::leak`, `String::leak`) also compiles, but Miri
reports the leaked memory.

### Alternatives

The elided lifetime of the borrow in `add_word` can be written out. The two lifetimes are
independent.

```rust
fn add_word<'s, 'a>(seen: &'s mut Vec<&'a str>, word: &'a str)
```

You can also do without borrowed strings in `merge` by collecting owned `String`s from the start, and
let `unique_upper` remember the words it has seen in a `HashSet<&str>`. `insert` returns `false` for a
word that is already in the set, so it fits `filter` directly, with no helper and no lifetime
annotations at all.

```rust
use std::collections::HashSet;

pub fn merge(defaults: &[&'static str], extra: &str) -> Vec<String> {
    let mut all: Vec<String> = defaults.iter().map(|d| d.to_string()).collect();
    for word in extra.split(',').map(str::trim).filter(|w| !w.is_empty()) {
        if !all.iter().any(|a| a == word) {
            all.push(word.to_string());
        }
    }
    all
}

pub fn unique_upper(text: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    text.split_whitespace()
        .filter(|word| seen.insert(*word))
        .map(str::to_uppercase)
        .collect()
}
```
