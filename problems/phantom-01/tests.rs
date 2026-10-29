use crate::judge::{case, check_eq};
use crate::user::{Inch, Length, Mm, perimeter, to_mm};

/// Units of the tests' own. Like `Mm` and `Inch`, they derive `Debug`, `Clone`, `Copy` and
/// `PartialEq`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Cubit;
#[derive(Debug, Clone, Copy, PartialEq)]
enum Furlongs {}

/// Generic code that knows nothing about `U`: the number is on the left.
fn triple<U>(l: Length<U>) -> Length<U> {
    3 * l
}

fn halve<U>(l: Length<U>) -> Length<U> {
    0.5 * l
}

pub fn run() {
    case("example-1", || {
        let a = Length::<Mm>::new(2.0);
        check_eq(a + a, Length::new(4.0))?;
        check_eq(a * 3, Length::new(6.0))?;
        check_eq(3 * a, Length::new(6.0))?;
        check_eq(perimeter(a, Length::new(3.0)), Length::new(10.0))?;
        check_eq(a.value(), 2.0)
    });
    case("example-2", || {
        let l = Length::<Inch>::new(2.0);
        check_eq(2.5 * l, Length::new(5.0))?;
        check_eq(to_mm(l).value(), 2.0 * 25.4)?;
        check_eq(Length::<Mm>::new(6.0) / Length::new(1.5), 4.0)
    });
    case("case-1", || {
        // Every number type on the left, with a unit of the tests.
        let c = Length::<Cubit>::new(1.5);
        check_eq(2.0f64 * c, Length::new(3.0))?;
        check_eq(0.5f32 * c, Length::new(0.75))?;
        check_eq(-4i32 * c, Length::new(-6.0))?;
        check_eq(4u32 * c, Length::new(6.0))?;
        check_eq(0 * c, Length::new(0.0))?;
        check_eq(c.value(), 1.5)
    });
    case("case-2", || {
        // Both sides give the same result, also at the edges of the integer types.
        let l = Length::<Furlongs>::new(0.25);
        check_eq(u32::MAX * l, l * u32::MAX)?;
        check_eq((u32::MAX * l).value(), 4_294_967_295.0 * 0.25)?;
        check_eq(i32::MIN * l, Length::new(-536_870_912.0))?;
        check_eq(1.25f32 * l, l * 1.25f32)?;
        // The product is computed in f64, not in f32.
        check_eq(0.1f32 * Length::<Cubit>::new(3.0), Length::new(f64::from(0.1f32) * 3.0))?;
        check_eq(-2.0 * l, l * -2.0)?;
        // On the right, any number type that converts into `f64` works.
        check_eq(l * 2u8, Length::new(0.5))?;
        check_eq(l * -3i16, Length::new(-0.75))
    });
    case("case-3", || {
        // Generic code that knows nothing about the unit.
        check_eq(triple(Length::<Cubit>::new(0.5)), Length::new(1.5))?;
        check_eq(halve(Length::<Furlongs>::new(3.0)), Length::new(1.5))?;
        check_eq(perimeter(Length::<Furlongs>::new(1.25), Length::new(0.5)), Length::new(3.5))
    });
    case("case-4", || {
        // perimeter, + and /; the arguments stay usable.
        let w = Length::<Cubit>::new(4.0);
        let h = Length::new(-1.0);
        check_eq(perimeter(w, h), Length::new(6.0))?;
        check_eq((w.value(), h.value()), (4.0, -1.0))?;
        check_eq(perimeter(w, w) / w, 4.0)?;
        check_eq(w + h + w, Length::new(7.0))?;
        check_eq(Length::<Mm>::new(1.0) / Length::new(4.0), 0.25)
    });
    case("case-5", || {
        // to_mm multiplies by 25.4, and the result is an ordinary Length<Mm>.
        check_eq(to_mm(Length::new(0.0)), Length::<Mm>::new(0.0))?;
        check_eq(to_mm(Length::new(10.0)), Length::new(10.0 * 25.4))?;
        let total: Length<Mm> = to_mm(Length::new(1.0)) + 2 * Length::new(4.5);
        check_eq(total.value(), 25.4 + 9.0)
    });
}
