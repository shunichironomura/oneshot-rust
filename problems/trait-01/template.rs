pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;

    fn describe(&self) -> String {
        todo!()
    }
}

pub struct Rect {
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}

pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    todo!()
}
