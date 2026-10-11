use std::ops::{Add, AddAssign, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    pub fn dot(self, other: Vec2<T>) -> T {
        todo!()
    }
}

impl<T> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        todo!()
    }
}

impl<T> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        todo!()
    }
}

impl<T> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, k: T) -> Vec2<T> {
        todo!()
    }
}

impl<T> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Vec2<T> {
        todo!()
    }
}
