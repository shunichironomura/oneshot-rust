`StepRange` counts up from `start` in steps of `step` and stops before `end`. It can be walked from
both ends, and it always knows how many items are left. Implement it.

```rust
pub struct StepRange { /* up to you */ }

impl StepRange {
    pub fn new(start: i64, end: i64, step: u64) -> StepRange;
}

impl Iterator for StepRange {
    type Item = i64;
    // ...
}

impl DoubleEndedIterator for StepRange { /* ... */ }

impl ExactSizeIterator for StepRange {}
```

The fields of `StepRange` and the implementation are up to you (the skeleton is already in the
editor).

- The items are `start`, `start + step`, `start + 2 * step`, ... as long as they are less than `end`
  (`end` itself is never an item). They are exact integers that never wrap around. If
  `start >= end`, the range is empty.
- `next` yields the items from the front, in ascending order. `next_back` yields them from the back,
  starting with the largest item and going down.
- Calls to `next` and `next_back` can be mixed: every item is yielded exactly once, whichever end it
  comes from. Once all items have been yielded, both methods return `None`, and keep doing so.
- `len()` is the number of items not yet yielded, and `size_hint()` is exact: at any time it returns
  `(len, Some(len))`.
- `new` panics if `step` is `0`, even when the range is empty. Nothing else may panic.
- Every `start`, `end` and `step` must work, including `i64::MIN`, `i64::MAX` and `u64::MAX`. A
  range can have up to `u64::MAX` items, so compute the items on demand: `new`, `next`, `next_back`
  and `size_hint` must each take constant time. The judge is a 64-bit target, so `usize` is 64 bits
  wide.

### Examples

```rust
StepRange::new(-10, 10, 7).collect::<Vec<_>>()       // [-10, -3, 4]
StepRange::new(-10, 10, 7).rev().collect::<Vec<_>>() // [4, -3, -10]

let mut r = StepRange::new(-10, 10, 7);
r.next() // Some(-10)
r.len()  // 2

StepRange::new(i64::MAX - 5, i64::MAX, 4).collect::<Vec<_>>()    // [i64::MAX - 5, i64::MAX - 1]
StepRange::new(i64::MIN, i64::MAX, u64::MAX).collect::<Vec<_>>() // [i64::MIN]
```
