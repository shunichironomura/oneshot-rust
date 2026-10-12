use std::fmt::Display;

/// A point whose two coordinates may have different types.
#[derive(Debug, PartialEq)]
pub struct Point<X, Y> {
    pub x: X,
    pub y: Y,
}

impl<X, Y> Point<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Point { x, y }
    }

    pub fn convert<X2, Y2>(self) -> Point<X2, Y2>
    where
        X: Into<X2>,
        Y: Into<Y2>,
    {
        Point { x: self.x.into(), y: self.y.into() }
    }
}

impl Point<f64, f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl<T: PartialOrd + Display> Point<T, T> {
    pub fn larger(&self) -> String {
        if self.x >= self.y {
            format!("x = {}", self.x)
        } else {
            format!("y = {}", self.y)
        }
    }
}
