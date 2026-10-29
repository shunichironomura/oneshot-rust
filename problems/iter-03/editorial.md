`position` returns how many items came before the match in the iterator it is called on, so after
`rev()` it counts from the back. `words.iter().rev().position(|w| w.chars().count() >= 5)` returns
`Some(0)` for `["tea", "coffee", "milk", "juice"]`, because `"juice"` is the first item of the
reversed iterator. `rposition` searches from the back too, but returns the position from the front.
It needs an iterator that knows its length and can be walked from the back (`ExactSizeIterator` and
`DoubleEndedIterator`), as `slice::Iter` does. Count with `chars().count()`: `len()` is the length
in bytes, and `"日本"` is 6 bytes long but only 2 characters.

### Alternatives

`enumerate` before `rev` attaches the positions from the front, and they stay attached when the
order is reversed. (`rev().enumerate()` would number the items from the back again.)

```rust
pub fn last_long(words: &[&str], min: usize) -> Option<usize> {
    words
        .iter()
        .enumerate()
        .rev()
        .find(|(_, w)| w.chars().count() >= min)
        .map(|(i, _)| i)
}
```
