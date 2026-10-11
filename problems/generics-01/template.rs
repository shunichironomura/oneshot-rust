/// A point whose two coordinates may have different types.
#[derive(Debug, PartialEq)]
pub struct Point<X, Y> {
    pub x: X,
    pub y: Y,
}

impl Point {
    pub fn new(x: X, y: Y) -> Self {
        Point { x, y }
    }

    pub fn mixup(self, other: Point) -> Point {
        Point { x: self.x, y: other.y }
    }

    pub fn swap(self) -> Point {
        Point { x: self.y, y: self.x }
    }

    pub fn convert(self) -> Point {
        Point { x: self.x.into(), y: self.y.into() }
    }
}

impl Point {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Point {
    pub fn larger(&self) -> String {
        if self.x >= self.y {
            format!("x = {}", self.x)
        } else {
            format!("y = {}", self.y)
        }
    }
}
