use crate::judge::{case, check_eq};
use crate::user::quadrant;

pub fn run() {
    case("example-1", || {
        check_eq(quadrant(-2, 5), "II")?;
        check_eq(quadrant(0, -3), "y-axis")
    });
    case("case-1", || {
        check_eq(
            [quadrant(3, 4), quadrant(-3, -4), quadrant(3, -4), quadrant(i32::MIN, i32::MAX)],
            ["I", "III", "IV", "II"],
        )
    });
    case("case-2", || {
        // Points on the axes belong to no quadrant.
        check_eq(
            [quadrant(0, 0), quadrant(0, 5), quadrant(0, i32::MIN), quadrant(-3, 0), quadrant(7, 0)],
            ["origin", "y-axis", "y-axis", "x-axis", "x-axis"],
        )
    });
}
