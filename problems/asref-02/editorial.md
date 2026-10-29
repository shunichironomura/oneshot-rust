The body calls `into_iter()` twice and `as_ref()` once, so the signature needs one bound per level:
the argument is `IntoIterator`, each of its items is `IntoIterator` again, and each of their items
is `AsRef<str>`. `IntoIterator`, not `Iterator`, is the right trait: a `Vec` or an array is not an
iterator, but it can be turned into one, and every iterator is `IntoIterator` too.

The solution writes the bounds with associated type bounds (stable since Rust 1.79):
`I: IntoIterator<Item: IntoIterator<Item: AsRef<str>>>`. Iterating over `&[Vec<String>; 1]` yields
`&Vec<String>`, whose items are `&String`. `&String` is `AsRef<str>` as well, through the blanket
impl `impl<T: ?Sized + AsRef<U>, U: ?Sized> AsRef<U> for &T`, so references work without any extra
code.

The bound is also what makes `s.as_ref()` unambiguous. In the template, `s: String` gives `as_ref`
several candidates (`String` implements `AsRef<str>`, `AsRef<[u8]>`, `AsRef<OsStr>` and
`AsRef<Path>`), so even the body fails with "type annotations needed". In the generic function, all
that is known about the inner item is that it is `AsRef<str>`, so `as_ref` returns `&str`.

### Alternatives

Without associated type bounds, write one bound per level in a `where` clause. The inner item type
must be written `<I::Item as IntoIterator>::Item`; `I::Item::Item` is rejected as an ambiguous
associated type.

```rust
pub fn total_chars<I>(groups: I) -> usize
where
    I: IntoIterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}
```

Or give every level its own type parameter.

```rust
pub fn total_chars<I, G, S>(groups: I) -> usize
where
    I: IntoIterator<Item = G>,
    G: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}
```

`impl Trait` in argument position is shorter.

```rust
pub fn total_chars(groups: impl IntoIterator<Item = impl IntoIterator<Item = impl AsRef<str>>>) -> usize {
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}
```
