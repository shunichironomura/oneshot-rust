use std::fmt::Write;

/// Hands out items one at a time, like an iterator. An item may borrow from the lender itself,
/// so it has to be dropped before `next` is called again.
pub trait Lender {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

/// The overlapping windows of `size` consecutive elements of a slice, from left to right.
pub struct WindowsMut<'s, T> {
    slice: &'s mut [T],
    size: usize,
    pos: usize,
}

/// Panics if `size` is 0.
pub fn windows_mut<T>(slice: &mut [T], size: usize) -> WindowsMut<'_, T> {
    assert!(size > 0, "the window size must be at least 1");
    WindowsMut { slice, size, pos: 0 }
}

impl<'s, T> Lender for WindowsMut<'s, T> {
    type Item<'a> = &'a mut [T] where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let window = self.slice.get_mut(self.pos..self.pos + self.size)?;
        self.pos += 1;
        Some(window)
    }
}

/// The lines of a text, each with its line number in front ("1: first line"), written into one
/// buffer that is reused for every line.
pub struct Numbered<'t> {
    lines: std::str::Lines<'t>,
    n: usize,
    buf: String,
}

pub fn numbered(text: &str) -> Numbered<'_> {
    Numbered { lines: text.lines(), n: 0, buf: String::new() }
}

impl<'t> Lender for Numbered<'t> {
    type Item<'a> = &'a str where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let line = self.lines.next()?;
        self.n += 1;
        self.buf.clear();
        write!(self.buf, "{}: {}", self.n, line).unwrap();
        Some(&self.buf)
    }
}

/// Calls `next` until it returns `None` and returns how many items there were.
pub fn count_all<L: Lender>(mut lender: L) -> usize {
    let mut n = 0;
    while lender.next().is_some() {
        n += 1;
    }
    n
}
