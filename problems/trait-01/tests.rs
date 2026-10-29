use crate::judge::{case, check, check_eq};
use crate::user::{Circle, Rect, Shape, total_area};
use std::f64::consts::PI;

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }

    fn name(&self) -> String {
        "square".to_string()
    }
}

fn close(got: f64, expected: f64) -> Result<(), String> {
    check(
        (got - expected).abs() < 1e-9,
        &format!("got {got}, expected {expected}"),
    )
}

pub fn run() {
    case("example-1", || {
        let r = Rect { width: 2.0, height: 3.0 };
        close(r.area(), 6.0)?;
        check_eq(r.name(), "rect")?;
        check_eq(r.describe(), "rect with area 6.00")
    });
    case("example-2", || {
        let r = Rect { width: 2.0, height: 3.0 };
        let c = Circle { radius: 1.0 };
        close(total_area(&[&r, &c]), 6.0 + PI)
    });
    case("case-1", || {
        let c = Circle { radius: 2.0 };
        close(c.area(), 4.0 * PI)?;
        check_eq(c.name(), "circle")?;
        check_eq(c.describe(), "circle with area 12.57")
    });
    case("case-2", || close(total_area(&[]), 0.0));
    case("case-3", || {
        let s = Square(1.5);
        check_eq(s.describe(), "square with area 2.25")?;
        let r = Rect { width: 0.5, height: 4.0 };
        close(total_area(&[&s, &r, &s]), 6.5)
    });
    case("case-4", || {
        let r = Rect { width: 0.0, height: 10.0 };
        check_eq(r.describe(), "rect with area 0.00")
    });
}
