use std::marker::PhantomData;
use std::ops::{Add, Div, Mul};

/// Millimeters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mm;
/// Inches.
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// `a + b`: the sum of two lengths in the same unit.
impl<U> Add for Length<U> {
    type Output = Length<U>;

    fn add(self, other: Length<U>) -> Length<U> {
        Length::new(self.value + other.value)
    }
}

/// `a / b`: how many times `b` fits into `a`, a plain number.
impl<U> Div for Length<U> {
    type Output = f64;

    fn div(self, other: Length<U>) -> f64 {
        self.value / other.value
    }
}

/// `l * k`: the length scaled by the number `k`.
impl<U, K: Into<f64>> Mul<K> for Length<U> {
    type Output = Length<U>;

    fn mul(self, k: K) -> Length<U> {
        Length::new(self.value * k.into())
    }
}

// `k * l`, with the number on the left. `impl<U, K> Mul<Length<U>> for K` breaks the orphan
// rule (E0210), so each number type gets its own impl. `Length<U>` is a local type, so these
// impls are allowed even though both `Mul` and the number types come from the standard library.
impl<U> Mul<Length<U>> for f64 {
    type Output = Length<U>;

    fn mul(self, l: Length<U>) -> Length<U> {
        l * self
    }
}

impl<U> Mul<Length<U>> for f32 {
    type Output = Length<U>;

    fn mul(self, l: Length<U>) -> Length<U> {
        l * self
    }
}

impl<U> Mul<Length<U>> for i32 {
    type Output = Length<U>;

    fn mul(self, l: Length<U>) -> Length<U> {
        l * self
    }
}

impl<U> Mul<Length<U>> for u32 {
    type Output = Length<U>;

    fn mul(self, l: Length<U>) -> Length<U> {
        l * self
    }
}

/// The perimeter of a `w` x `h` rectangle.
pub fn perimeter<U>(w: Length<U>, h: Length<U>) -> Length<U> {
    2 * (w + h)
}

pub fn to_mm(l: Length<Inch>) -> Length<Mm> {
    Length::new(l.value() * 25.4)
}
