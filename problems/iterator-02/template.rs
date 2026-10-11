pub struct StepRange {
    // Add the fields you need.
}

impl StepRange {
    pub fn new(start: i64, end: i64, step: u64) -> StepRange {
        todo!()
    }
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        todo!()
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<i64> {
        todo!()
    }
}

impl ExactSizeIterator for StepRange {}
