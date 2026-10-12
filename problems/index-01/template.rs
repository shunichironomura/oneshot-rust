use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    /// The cells row by row: cell (x, y) is stored at index y * width + x.
    cells: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        todo!()
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        todo!()
    }

    /// Row `y` as a slice of `width` cells. Panics if `y >= height`.
    pub fn row(&self, y: usize) -> &[T] {
        assert!(y < self.height, "row {y} is outside a grid of height {}", self.height);
        let start = y * self.width;
        &self.cells[start..start + self.width]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        todo!()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        todo!()
    }
}
