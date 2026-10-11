use std::mem;

pub struct RunsMut<'a, T> {
    /// The part of the slice whose runs have not been yielded yet.
    rest: &'a mut [T],
}

pub fn runs_mut<T: PartialEq>(v: &mut [T]) -> RunsMut<'_, T> {
    RunsMut { rest: v }
}

impl<'a, T: PartialEq> Iterator for RunsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.rest.is_empty() {
            return None;
        }
        // The first element always starts the run; each neighbor that compares equal extends it.
        let len = 1 + self.rest.windows(2).take_while(|w| w[0] == w[1]).count();
        // Move the `&'a mut [T]` out of `self`, leaving an empty slice behind, so that the two
        // halves from `split_at_mut` keep the full lifetime `'a`.
        let rest = mem::take(&mut self.rest);
        let (run, tail) = rest.split_at_mut(len);
        self.rest = tail;
        Some(run)
    }
}

impl<'a, T: PartialEq> DoubleEndedIterator for RunsMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        if self.rest.is_empty() {
            return None;
        }
        let len = 1 + self.rest.windows(2).rev().take_while(|w| w[0] == w[1]).count();
        let rest = mem::take(&mut self.rest);
        let (head, run) = rest.split_at_mut(rest.len() - len);
        self.rest = head;
        Some(run)
    }
}
