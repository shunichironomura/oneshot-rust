use crate::judge::{case, check_eq};
use crate::user::{max_reading, sorted_readings, spread};

const NAN: f64 = f64::NAN;
const INF: f64 = f64::INFINITY;
const NEG_INF: f64 = f64::NEG_INFINITY;

pub fn run() {
    case("example-1", || {
        check_eq(max_reading(&[1.5, NAN, -2.0]), Some(1.5))?;
        check_eq(max_reading(&[NAN]), None)
    });
    case("example-2", || {
        check_eq(sorted_readings(&[3.0, NAN, -1.0, 2.5]), vec![-1.0, 2.5, 3.0])
    });
    case("example-3", || {
        check_eq(spread(&[4.0, NAN, 1.5]), Some(2.5))?;
        check_eq(spread(&[2.0]), Some(0.0))
    });
    case("case-1", || {
        // No readings at all.
        check_eq(max_reading(&[]), None)?;
        check_eq(sorted_readings(&[]), Vec::<f64>::new())?;
        check_eq(spread(&[]), None)
    });
    case("case-2", || {
        // Only failed readings, including a negative NaN.
        let v = [NAN, -NAN, NAN];
        check_eq(max_reading(&v), None)?;
        check_eq(sorted_readings(&v), Vec::<f64>::new())?;
        check_eq(spread(&v), None)
    });
    case("case-3", || {
        // Every reading is negative.
        let v = [-1.0, NAN, -3.0];
        check_eq(max_reading(&v), Some(-1.0))?;
        check_eq(sorted_readings(&v), vec![-3.0, -1.0])?;
        check_eq(spread(&v), Some(2.0))
    });
    case("case-4", || {
        // A NaN first must not hide the other readings.
        let v = [NAN, 2.0, 7.0, NAN];
        check_eq(max_reading(&v), Some(7.0))?;
        check_eq(spread(&v), Some(5.0))?;
        check_eq(max_reading(&[-NAN, -4.0]), Some(-4.0))?;
        check_eq(spread(&[5.0, -NAN, 3.0]), Some(2.0))
    });
    case("case-5", || {
        // Infinities are ordinary readings, and the subtraction may overflow to infinity.
        check_eq(max_reading(&[INF, 1.0]), Some(INF))?;
        check_eq(max_reading(&[NEG_INF, NAN]), Some(NEG_INF))?;
        check_eq(spread(&[NEG_INF, 0.0]), Some(INF))?;
        check_eq(spread(&[f64::MAX, f64::MIN]), Some(INF))
    });
    case("case-6", || {
        // Duplicates are kept, and infinities sort to the ends.
        check_eq(
            sorted_readings(&[INF, -1.0, NEG_INF, NAN, 0.5, -1.0, 0.5]),
            vec![NEG_INF, -1.0, -1.0, 0.5, 0.5, INF],
        )
    });
}
