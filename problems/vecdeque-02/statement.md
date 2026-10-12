Implement `josephus`, which solves an old counting-off puzzle.

```rust
pub fn josephus(n: usize, k: usize) -> Vec<usize>
```

- `n` people, numbered `1` to `n`, stand in a circle in that order (person `1` comes after person
  `n`). Starting with person `1`, they count off `1`, `2`, `3`, … around the circle. The person
  who says `k` leaves the circle, and the next person still in it starts again from `1`. This
  repeats until everyone has left.
- It returns the numbers of the people in the order they leave.
- `n` may be `0`; the result is then empty. `k` is at least `1` and may be any `usize` up to
  `usize::MAX`, and the result must come quickly even for such a large `k`.

You may use anything from `std`, for example `std::collections::VecDeque`, a queue that can add and
remove elements at both ends.

### Examples

`josephus(7, 3)` returns `[3, 6, 2, 7, 5, 1, 4]`. After 3 and 6 have left, person 7 says `1`,
person 1 says `2` and person 2 says `3`, so person 2 leaves next.
