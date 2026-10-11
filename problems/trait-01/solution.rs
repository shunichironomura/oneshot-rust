use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;

    fn describe(&self) -> String {
        format!("{} with area {:.2}", self.name(), self.area())
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
        self.width * self.height
    }

    fn name(&self) -> String {
        String::from("rect")
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn name(&self) -> String {
        String::from("circle")
    }
}

pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}
