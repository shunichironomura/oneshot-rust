use crate::judge::{case, check_eq};
use crate::user::Vec2;

pub fn run() {
    case("example-1", || {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, -4);
        check_eq(a + b, Vec2::new(4, -2))?;
        check_eq(a * 3, Vec2::new(3, 6))?;
        check_eq(-b, Vec2::new(-3, 4))?;
        check_eq(a.dot(b), -5)
    });
    case("example-2", || {
        let mut p = Vec2::new(0.5, 1.0);
        p += Vec2::new(0.25, -2.0);
        check_eq(p, Vec2::new(0.75, -1.0))?;
        check_eq(-(p * 2.0), Vec2::new(-1.5, 2.0))
    });
    case("case-1", || {
        let a = Vec2::new(2, 5);
        let b = Vec2::new(-1, 3);
        // Operators chain and keep their usual precedence.
        check_eq(a + b * 2 + -a, Vec2::new(-2, 6))?;
        check_eq((a + b).dot(a + b), 1 + 64)
    });
    case("case-2", || {
        let mut sum = Vec2::new(0i64, 0i64);
        for i in 1..=4i64 {
            sum += Vec2::new(i, i * i);
        }
        check_eq(sum, Vec2::new(10, 30))?;
        check_eq(sum * 0, Vec2::new(0, 0))
    });
    case("case-3", || {
        let v = Vec2::new(3.0f32, 4.0f32);
        check_eq(v.dot(v), 25.0)?;
        check_eq(-v * -1.0, v)?;
        check_eq(v + Vec2::new(-3.0, -4.0), Vec2::new(0.0, 0.0))
    });
    case("case-4", || {
        let a = Vec2::new(1.5f64, -2.0);
        let b = Vec2::new(-4.0f64, 0.25);
        check_eq(a.dot(b), -6.5)?;
        check_eq(a * 0.5 + b, Vec2::new(-3.25, -0.75))
    });
}
