use crate::judge::{case, check_eq};
use crate::user::{Celsius, Fahrenheit, average, is_freezing, warmest};

pub fn run() {
    case("example-1", || {
        check_eq(Celsius::from(Fahrenheit(212.0)), Celsius(100.0))?;
        let f: Fahrenheit = Celsius(-40.0).into();
        check_eq(f, Fahrenheit(-40.0))
    });
    case("example-2", || {
        check_eq(is_freezing(Fahrenheit(32.0)), true)?;
        check_eq(is_freezing(Celsius(0.5)), false)
    });
    case("example-3", || {
        check_eq(average(vec![Fahrenheit(32.0), Fahrenheit(212.0)]), Some(Celsius(50.0)))?;
        check_eq(average(vec![Celsius(10.0), Celsius(20.0)]), Some(Celsius(15.0)))?;
        check_eq(
            warmest(&[Fahrenheit(50.0), Fahrenheit(212.0), Fahrenheit(32.0)]),
            Some(Celsius(100.0)),
        )
    });
    case("case-1", || {
        check_eq(Fahrenheit::from(Celsius(100.0)), Fahrenheit(212.0))?;
        check_eq(Fahrenheit::from(Celsius(0.0)), Fahrenheit(32.0))?;
        check_eq(Celsius::from(Fahrenheit(14.0)), Celsius(-10.0))?;
        let c: Celsius = Fahrenheit(-4.0).into();
        check_eq(c, Celsius(-20.0))?;
        check_eq(Celsius::from(Fahrenheit::from(Celsius(35.0))), Celsius(35.0))
    });
    case("case-2", || {
        // 0 °C itself is freezing; the comparison happens in Celsius.
        check_eq(is_freezing(Celsius(0.0)), true)?;
        check_eq(is_freezing(Celsius(-10.0)), true)?;
        check_eq(is_freezing(Celsius(25.0)), false)?;
        check_eq(is_freezing(Fahrenheit(31.0)), true)?;
        check_eq(is_freezing(Fahrenheit(33.0)), false)?;
        check_eq(is_freezing(Fahrenheit(50.0)), false)
    });
    case("case-3", || {
        // An empty list has no average (not NaN).
        check_eq(average(Vec::<Celsius>::new()), None)?;
        check_eq(average(Vec::<Fahrenheit>::new()), None)
    });
    case("case-4", || {
        check_eq(average(vec![Fahrenheit(-4.0)]), Some(Celsius(-20.0)))?;
        check_eq(
            average(vec![Fahrenheit(-40.0), Fahrenheit(50.0), Fahrenheit(104.0), Fahrenheit(-4.0)]),
            Some(Celsius(-2.5)),
        )?;
        check_eq(average(vec![Celsius(-5.0), Celsius(0.0), Celsius(20.0)]), Some(Celsius(5.0)))
    });
    case("case-5", || {
        // Every reading is below zero.
        check_eq(warmest(&[Fahrenheit(-40.0), Fahrenheit(14.0)]), Some(Celsius(-10.0)))?;
        check_eq(
            warmest(&[Fahrenheit(14.0), Fahrenheit(-40.0), Fahrenheit(-4.0)]),
            Some(Celsius(-10.0)),
        )?;
        check_eq(warmest(&[Fahrenheit(5.0)]), Some(Celsius(-15.0)))
    });
    case("case-6", || {
        check_eq(warmest(&[]), None)?;
        check_eq(
            warmest(&[Fahrenheit(32.0), Fahrenheit(68.0), Fahrenheit(104.0)]),
            Some(Celsius(40.0)),
        )
    });
}
