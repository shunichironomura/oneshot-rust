`HashSet::insert` answers two questions at once: it adds the value, and it returns `true` when the
value is new and `false` when it is already in the set. So there is no need to call `contains`
before `insert`.

In `first_repeat`, the first word whose `insert` returns `false` is the answer.
`for &word in words` destructures each `&&'a str` into a `&'a str`, so the word can be returned as
it is: it borrows from the original strings, not from `words` or from the set.

In `all_unique`, every `insert` must return `true`. `Iterator::all` stops at the first `false`, and
its closure may modify `seen`, because `all` accepts an `FnMut` closure.

### Alternatives

`first_repeat` also fits in one expression, with `Iterator::find`.

```rust
pub fn first_repeat<'a>(words: &[&'a str]) -> Option<&'a str> {
    let mut seen = HashSet::new();
    words.iter().copied().find(|&word| !seen.insert(word))
}
```

Once `first_repeat` exists, `all_unique` is just its `None` case.

```rust
pub fn all_unique(words: &[&str]) -> bool {
    first_repeat(words).is_none()
}
```
