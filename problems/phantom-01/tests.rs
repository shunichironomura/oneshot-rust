use crate::judge::{case, check, check_eq};
use crate::user::{Inch, Length, Mm, longest, perimeter, to_mm};

/// Units of the tests' own. They derive nothing and implement no traits.
struct Cubit;
enum Furlongs {}

/// Generic code that knows nothing about `U`: copies, compares and prints lengths.
fn grow<U>(l: Length<U>, times: u32) -> Length<U> {
    let mut total = l;
    for _ in 1..times {
        total = total + l;
    }
    total
}

fn same<U>(a: &Length<U>, b: &Length<U>) -> Result<(), String> {
    check_eq(a.clone(), *b)
}

pub fn run() {
    case("sample-1", || {
        let a = Length::<Mm>::new(2.0);
        let p = perimeter(a, Length::new(3.0));
        check_eq(p, Length::new(10.0))?;
        check_eq(a.value(), 2.0)?;
        check_eq(format!("{p:?}"), "Length(10.0)")
    });
    case("sample-2", || {
        check_eq(to_mm(Length::<Inch>::new(2.0)).value(), 2.0 * 25.4)?;
        let items = [Length::<Inch>::new(1.5), Length::new(4.0), Length::new(2.0)];
        check_eq(longest(&items), Some(Length::new(4.0)))
    });
    case("hidden-1", || {
        // perimeter and + with a unit that implements nothing; the arguments stay usable.
        let w = Length::<Cubit>::new(1.5);
        let h = Length::<Cubit>::new(0.25);
        let p = perimeter(w, h);
        check_eq(p, Length::new(3.5))?;
        check_eq((w.value(), h.value()), (1.5, 0.25))?;
        check_eq(perimeter(p, w), Length::new(10.0))?;
        check_eq(w + h + w, Length::new(3.25))
    });
    case("hidden-2", || {
        // longest: empty, a single element, only negative values; the slice is unchanged.
        check_eq(longest::<Furlongs>(&[]), None)?;
        check_eq(longest(&[Length::<Furlongs>::new(7.25)]), Some(Length::new(7.25)))?;
        let neg = [Length::<Furlongs>::new(-3.0), Length::new(-0.5), Length::new(-2.0)];
        check_eq(longest(&neg), Some(Length::new(-0.5)))?;
        let v = vec![
            Length::<Cubit>::new(1.0),
            Length::new(9.5),
            Length::new(9.0),
            Length::new(3.0),
        ];
        let best = longest(&v);
        check_eq(best, Some(v[1]))?;
        check_eq(v, vec![Length::new(1.0), Length::new(9.5), Length::new(9.0), Length::new(3.0)])
    });
    case("hidden-3", || {
        // Copies and clones are equal; equality compares the values as f64.
        let a = Length::<Cubit>::new(4.0);
        let b = a;
        let c = a.clone();
        check_eq(a, b)?;
        check_eq(b, c)?;
        check(a != Length::new(4.5), "Length(4.0) should not equal Length(4.5)")?;
        check(!(a == Length::new(-4.0)), "Length(4.0) should not equal Length(-4.0)")?;
        check(Length::<Cubit>::new(1e-20) != Length::new(0.0), "1e-20 should not equal 0.0")?;
        check_eq(Length::<Furlongs>::new(0.0), Length::new(-0.0))?;
        let copies = vec![Length::<Cubit>::new(1.25); 3];
        check_eq(copies[0] + copies[1] + copies[2], Length::new(3.75))
    });
    case("hidden-4", || {
        // Generic code without any bound on the unit.
        let a = Length::<Furlongs>::new(0.5);
        check_eq(grow(a, 4), Length::new(2.0))?;
        same(&grow(Length::<Cubit>::new(3.0), 1), &Length::new(3.0))?;
        same(&a, &a)
    });
    case("hidden-5", || {
        // {:?} prints the value with {:?} inside Length(...).
        check_eq(format!("{:?}", Length::<Furlongs>::new(2.5)), "Length(2.5)")?;
        check_eq(format!("{:?}", Length::<Cubit>::new(-0.75)), "Length(-0.75)")?;
        check_eq(format!("{:?}", Some(Length::<Mm>::new(1.0))), "Some(Length(1.0))")
    });
    case("hidden-6", || {
        // to_mm multiplies by 25.4, and the result is an ordinary Length<Mm>.
        check_eq(to_mm(Length::new(0.0)), Length::<Mm>::new(0.0))?;
        check_eq(to_mm(Length::new(10.0)), Length::new(10.0 * 25.4))?;
        let total = to_mm(Length::new(1.0)) + Length::<Mm>::new(4.5);
        check_eq(total.value(), 25.4 + 4.5)
    });
}
