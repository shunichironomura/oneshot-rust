With two reference parameters, the compiler cannot guess which one the `&str` items borrow from, so
the lifetime has to be named: the lines come from `text`, hence `text: &'a str` and
`Item = &'a str`. `pattern` is copied into an owned `String` right away, so the iterator does not
borrow it at all.

That alone is not enough. In Rust 2024, a returned `impl Trait` **captures every lifetime in scope**,
including the anonymous one of `pattern`. The caller must then keep `pattern` alive as long as the
iterator, and the harness, which drops it early, fails to compile. `+ use<'a>` lists exactly the
lifetimes the hidden type may use, which releases `pattern` as soon as the function returns.

`+ 'a`, the usual fix in older editions, does not help here: it only promises that the iterator is
valid for at least `'a`, and `pattern`'s lifetime is still captured.

### Alternatives

Naming the second lifetime makes it explicit that it is left out of `use<..>`.

```rust
pub fn matching_lines<'a, 'p>(
    text: &'a str,
    pattern: &'p str,
) -> impl Iterator<Item = &'a str> + use<'a> {
    let pattern = pattern.to_lowercase();
    text.lines()
        .filter(move |line| line.to_lowercase().contains(&pattern))
}
```

Note that `use<..>` must list every **type** parameter in scope; only lifetimes can be left out.
