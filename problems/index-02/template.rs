use std::ops::{Index, IndexMut, RangeBounds};

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    /// The cells row by row: cell (x, y) is stored at index y * width + x.
    cells: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        Grid { width, height, cells: vec![fill; width * height] }
    }
}

impl<T> Grid<T> {
    /// The position of (x, y) in `cells`, or `None` if either coordinate is out of range.
    fn offset(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.offset(x, y).map(|i| &self.cells[i])
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
        match self.offset(x, y) {
            Some(i) => &self.cells[i],
            None => panic!("({x}, {y}) is outside a {}x{} grid", self.width, self.height),
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        match self.offset(x, y) {
            Some(i) => &mut self.cells[i],
            None => panic!("({x}, {y}) is outside a {}x{} grid", self.width, self.height),
        }
    }
}

impl Grid<i32> {
    pub fn row_sum<R: RangeBounds<usize>>(&self, y: usize, cols: R) -> Option<i64> {
        todo!()
    }
}
