use std::marker::PhantomData;
use std::ops::Add;

/// Millimeters.
pub struct Mm;
/// Inches.
pub struct Inch;

/// A length in the unit `U`. The unit exists only in the type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length<U> {
    value: f64,
    unit: PhantomData<U>,
}

impl<U> Length<U> {
    pub fn new(value: f64) -> Self {
        Length { value, unit: PhantomData }
    }

    pub fn value(self) -> f64 {
        self.value
    }
}

impl<U> Add for Length<U> {
    type Output = Length<U>;

    fn add(self, other: Length<U>) -> Length<U> {
        Length::new(self.value + other.value)
    }
}

/// The perimeter of a `w` x `h` rectangle.
pub fn perimeter<U>(w: Length<U>, h: Length<U>) -> Length<U> {
    w + w + h + h
}

/// A copy of the longest length, or `None` if there is none.
pub fn longest<U>(items: &[Length<U>]) -> Option<Length<U>> {
    items.iter().copied().max_by(|a, b| a.value.total_cmp(&b.value))
}

pub fn to_mm(l: Length<Inch>) -> Length<Mm> {
    Length::new(l.value() * 25.4)
}
