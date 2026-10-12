A `VecDeque` is a ring buffer: adding or removing an element at **either end** takes O(1) time.
That makes it the natural type for a bounded history. The newest action goes in at the back with
`push_back`, `undo` takes it out again with `pop_back`, and when the history is over capacity the
oldest action falls out of the front with `pop_front`. With a `Vec`, `remove(0)` does the same job
but shifts every remaining element one place to the left.

In `record`, push first and then check the length. The other order, "if the history is full, drop
the oldest, then push", fails for a capacity of `0`: `pop_front` on an empty deque does nothing, and
the push then leaves one action behind.

`recent` walks backwards from the newest action: `iter().rev().take(n)`. The order of the adapters
matters. `take(n).rev()` would give the `n` *oldest* actions, reversed. `take` simply stops at the
end of the deque, so an `n` larger than `len()`, even `usize::MAX`, needs no special case. By
contrast, `Vec::with_capacity(n)` panics for `n = usize::MAX`, and `len() - n` underflows when `n`
is larger than `len()`.

The deque's iterator yields `&String`, and `map(|s| s.as_str())` turns each one into a `&str`
(`map(String::as_str)` also works). Deref coercion applies to a single reference, not to the items
of an iterator or a `Vec`, so the conversion has to be written out. Thanks to lifetime elision, the
returned `&str`s borrow from `&self`: `recent` and `oldest` copy no strings.

### Alternatives

`recent` can also start at the right index and walk backwards with `range`. `saturating_sub` stops
at `0` instead of underflowing when `n` is larger than `len()`.

```rust
pub fn recent(&self, n: usize) -> Vec<&str> {
    let start = self.actions.len().saturating_sub(n);
    self.actions.range(start..).rev().map(|s| s.as_str()).collect()
}
```
