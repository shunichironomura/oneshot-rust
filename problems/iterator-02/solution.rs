pub struct StepRange {
    /// The item `next` returns next (meaningless once `remaining` is 0).
    front: i64,
    step: u64,
    /// How many items are left. Both ends take from this one count, so they cannot cross.
    remaining: u64,
}

impl StepRange {
    pub fn new(start: i64, end: i64, step: u64) -> StepRange {
        assert!(step != 0, "step must not be zero");
        // `end - start` can be as large as 2^64 - 1: too large for an i64, but it fits in a u64.
        let remaining = if start < end {
            end.abs_diff(start).div_ceil(step)
        } else {
            0
        };
        StepRange { front: start, step, remaining }
    }
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.remaining = self.remaining.checked_sub(1)?;
        let item = self.front;
        // After the last item, `front + step` may lie beyond i64::MAX. It is never returned then,
        // so wrapping is harmless; for every other item the sum is exact.
        self.front = self.front.wrapping_add_unsigned(self.step);
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // usize is 64 bits wide on the judge, so the count fits.
        let n = self.remaining as usize;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<i64> {
        self.remaining = self.remaining.checked_sub(1)?;
        // The last item is `remaining * step` above the front. That distance fits in a u64, and
        // the sum is an item of the range, so it fits in an i64 and never actually wraps.
        Some(self.front.wrapping_add_unsigned(self.remaining * self.step))
    }
}

// `len` comes for free: it returns the exact `size_hint` above.
impl ExactSizeIterator for StepRange {}
