use crate::judge::{case, check_eq};
use crate::user::{digits_label, quadrant, sign_word};

pub fn run() {
    case("example-1", || {
        check_eq(digits_label(42), "two digits")?;
        check_eq(sign_word(-7), "negative")
    });
    case("example-2", || {
        check_eq(quadrant(-2, 5), "II")?;
        check_eq(quadrant(0, -3), "y-axis")
    });
    case("case-1", || {
        // Every boundary of u8, including 255.
        check_eq(
            [0, 9, 10, 99, 100, 254, 255].map(digits_label),
            [
                "one digit",
                "one digit",
                "two digits",
                "two digits",
                "three digits",
                "three digits",
                "three digits",
            ],
        )
    });
    case("case-2", || {
        check_eq(
            [i32::MIN, -1, 0, 1, i32::MAX].map(sign_word),
            ["negative", "negative", "zero", "positive", "positive"],
        )
    });
    case("case-3", || {
        check_eq(
            [quadrant(3, 4), quadrant(-3, -4), quadrant(3, -4), quadrant(i32::MIN, i32::MAX)],
            ["I", "III", "IV", "II"],
        )
    });
    case("case-4", || {
        // Points on the axes belong to no quadrant.
        check_eq(
            [quadrant(0, 0), quadrant(0, 5), quadrant(0, i32::MIN), quadrant(-3, 0), quadrant(7, 0)],
            ["origin", "y-axis", "y-axis", "x-axis", "x-axis"],
        )
    });
}
