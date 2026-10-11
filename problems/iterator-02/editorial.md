The solution stores three numbers: the next item from the front, the step, and how many items are
left. `next` and `next_back` both take one item from that single count, so the two ends cannot
cross, and `next_back` needs no position of its own: the last item is `remaining * step` above the
front (after the count has been decreased).

**`ExactSizeIterator` is built on `size_hint`.** It has no required methods. Its default `len()`
calls `size_hint()` and asserts that the lower and the upper bound agree, while the default
`size_hint()` of `Iterator` returns `(0, None)`. An empty `impl ExactSizeIterator` therefore panics
on the first `len()` unless you also implement `size_hint`. Overriding only `len` is not enough
either: `size_hint` stays wrong, and adapters such as `skip` and `take` compute their `len` from it.

**Overflow.** The range may cover all of `i64`, and every step is a `u64`:

- `end - start` can be `2^64 - 1`, which overflows an `i64` but fits in a `u64`:
  `end.abs_diff(start)`.
- The count is that distance divided by the step, rounded up. `(distance + step - 1) / step`
  overflows for large values; `u64::div_ceil` does not.
- `step as i64` turns a step of `2^63` or more into a negative number. `i64::wrapping_add_unsigned`
  (or `checked_add_unsigned`) adds a `u64` to an `i64` without the cast.
- Moving the front past the last item can overflow: in `new(i64::MAX - 5, i64::MAX, 4)`, the value
  after `i64::MAX - 1` lies beyond `i64::MAX`. The solution lets that one wrap, because it is never
  returned. Every value that is returned is an item of the range, so the addition that produces it
  never wraps.

`new` checks `step != 0` before anything else, so an empty range with a zero step panics too.

### Alternatives

std already has a stepping iterator that is double-ended and exact-size: `step_by` on a
`Range<usize>`. Step through the offsets `0..end - start` and add each one to `start`. `step_by`
itself panics when the step is `0`.

```rust
use std::iter::StepBy;
use std::ops::Range;

pub struct StepRange {
    start: i64,
    /// The distances of the remaining items from `start`.
    offsets: StepBy<Range<usize>>,
}

impl StepRange {
    pub fn new(start: i64, end: i64, step: u64) -> StepRange {
        // usize is 64 bits wide on the judge, so these conversions are lossless.
        let span = if start < end { end.abs_diff(start) as usize } else { 0 };
        // `step_by` panics if the step is 0.
        let offsets = (0..span).step_by(step as usize);
        StepRange { start, offsets }
    }

    fn item(&self, offset: usize) -> i64 {
        self.start.wrapping_add_unsigned(offset as u64)
    }
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.offsets.next().map(|offset| self.item(offset))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.offsets.size_hint()
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<i64> {
        self.offsets.next_back().map(|offset| self.item(offset))
    }
}

impl ExactSizeIterator for StepRange {}
```

`Range<i64>` would not work here: it is not an `ExactSizeIterator`, so `step_by` on it has no
`next_back`.

An `i128` holds every intermediate value, so a version that keeps a front item and a back item does
not have to think about overflow at all. The range is empty once the front has passed the back.

```rust
pub struct StepRange {
    /// The next item from the front and from the back. The range is empty when `front > back`.
    front: i128,
    back: i128,
    step: i128,
}

impl StepRange {
    pub fn new(start: i64, end: i64, step: u64) -> StepRange {
        assert!(step != 0, "step must not be zero");
        let (start, end, step) = (i128::from(start), i128::from(end), i128::from(step));
        // In i128, no difference or sum of these values can overflow.
        let back = if start < end {
            start + (end - 1 - start) / step * step
        } else {
            start - 1
        };
        StepRange { front: start, back, step }
    }
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.front > self.back {
            return None;
        }
        let item = self.front;
        self.front += self.step;
        Some(item as i64)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.front > self.back {
            0
        } else {
            ((self.back - self.front) / self.step + 1) as usize
        };
        (n, Some(n))
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<i64> {
        if self.front > self.back {
            return None;
        }
        let item = self.back;
        self.back -= self.step;
        Some(item as i64)
    }
}

impl ExactSizeIterator for StepRange {}
```
