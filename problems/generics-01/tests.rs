use std::fmt;

use crate::judge::{case, check_eq};
use crate::user::Point;

// A type of the harness that implements only PartialEq, PartialOrd and Display:
// it is not Copy, Clone, Debug, Eq or Ord.
#[derive(PartialEq, PartialOrd)]
struct Grade(u8);

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "grade {}", self.0)
    }
}

pub fn run() {
    case("example-1", || {
        check_eq(Point::new(1, "a"), Point { x: 1, y: "a" })?;
        let q: Point<f64, i64> = Point::new(3i32, 4u8).convert();
        check_eq(q, Point { x: 3.0, y: 4 })?;
        check_eq(
            Point::new("hi", 'A').convert::<String, u32>(),
            Point { x: String::from("hi"), y: 65 },
        )
    });
    case("example-2", || {
        check_eq(Point::new(3.0, 4.0).distance_from_origin(), 5.0)?;
        check_eq(Point::new('a', 'z').larger(), "y = z")?;
        check_eq(Point::new(7, 3).larger(), "x = 7")
    });
    case("case-1", || {
        // Coordinates that are not Copy, of two different types.
        let p = Point::new(String::from("left"), vec![Some(true)]);
        check_eq(p, Point { x: String::from("left"), y: vec![Some(true)] })
    });
    case("case-2", || {
        check_eq(Point::new(0.0, 0.0).distance_from_origin(), 0.0)?;
        let p: Point<f64, f64> = Point { x: -5.0, y: 12.0 };
        check_eq(p.distance_from_origin(), 13.0)?;
        // `distance_from_origin` only borrows the point.
        check_eq(p.distance_from_origin(), 13.0)?;
        check_eq(Point::new(-3.0, 4.0).distance_from_origin(), 5.0)
    });
    case("case-3", || {
        // On a tie, x is reported.
        check_eq(Point::new(4, 4).larger(), "x = 4")?;
        check_eq(Point::new(2.5, -1.0).larger(), "x = 2.5")?;
        check_eq(Point::new("b", "a").larger(), "x = b")?;
        let words = Point::new(String::from("apple"), String::from("pear"));
        check_eq(words.larger(), "y = pear")?;
        // `larger` only borrows the point.
        check_eq(words.x, "apple")
    });
    case("case-4", || {
        check_eq(Point::new(Grade(2), Grade(5)).larger(), "y = grade 5")?;
        check_eq(Point::new(Grade(7), Grade(3)).larger(), "x = grade 7")?;
        // `convert` also accepts coordinates that are neither Copy nor Clone.
        let q = Point::new(Grade(3), Grade(9)).convert::<Grade, Grade>();
        check_eq(q.larger(), "y = grade 9")
    });
    case("case-5", || {
        check_eq(Point::new(1u8, -2i16).convert::<f32, i64>(), Point { x: 1.0, y: -2 })?;
        check_eq(Point::new(7, 'x').convert::<i32, char>(), Point { x: 7, y: 'x' })?;
        check_eq(
            Point::new('z', String::from("abc")).convert::<String, String>(),
            Point { x: String::from("z"), y: String::from("abc") },
        )?;
        check_eq(Point::new(6u8, 8i32).convert::<f64, f64>().distance_from_origin(), 10.0)
    });
}
