Keep the circle in a `VecDeque` whose front is the person who says `1` next.
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

An index into a `Vec` works too. After `remove(pos)`, the next person moves into
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
