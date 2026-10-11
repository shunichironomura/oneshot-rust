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

For `josephus`, keep the circle in a `VecDeque` whose front is the person who says `1` next.
`rotate_left(m)` moves the first `m` elements to the back, in order. After `k - 1` people have
counted and moved to the back, the person who says `k` is at the front, and `pop_front` removes
them. The next person is then at the front, ready to start again from `1`.

The trap is in the documentation of `rotate_left`: it **panics** if `m` is greater than `len()`.
The counting may go around the circle many times, so of the `k - 1` steps only the remainder
`(k - 1) % len` matters. Even in the example, `rotate_left(k - 1)` without the remainder panics at
the very end, when only person 4 is left and `k - 1` is `2`. Two other ways to go wrong:

- `k % len - 1` underflows whenever `k` is a multiple of the circle's size.
- Moving people one at a time (`pop_front` then `push_back`, `k - 1` times) is correct, but takes
  `k` steps per person, so it never finishes for `k = usize::MAX`.

### Alternatives

`recent` can also start at the right index and walk backwards with `range`. `saturating_sub` stops
at `0` instead of underflowing when `n` is larger than `len()`.

```rust
pub fn recent(&self, n: usize) -> Vec<&str> {
    let start = self.actions.len().saturating_sub(n);
    self.actions.range(start..).rev().map(|s| s.as_str()).collect()
}
```

`josephus` can also use an index into a `Vec`. After `remove(pos)`, the next person moves into
`pos` (if `pos` was the last index, the next person is at `0`, and the `%` in the next round takes
care of that). Take the remainder of `k - 1` before adding it: `pos + k - 1` would overflow for
`k = usize::MAX`.

```rust
pub fn josephus(n: usize, k: usize) -> Vec<usize> {
    let mut circle: Vec<usize> = (1..=n).collect();
    let mut order = Vec::new();
    let mut pos = 0; // the index of the person who says 1
    while !circle.is_empty() {
        pos = (pos + (k - 1) % circle.len()) % circle.len();
        order.push(circle.remove(pos));
    }
    order
}
```
