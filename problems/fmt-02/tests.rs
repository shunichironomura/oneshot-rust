use crate::judge::{case, check_eq};
use crate::user::receipt_line;

pub fn run() {
    case("example-1", || {
        check_eq(receipt_line("Coffee", 2, 3.5), "Coffee       2 x     3.50")?;
        check_eq(receipt_line("Chocolate bar", 12, 1234.567), "Chocolate   12 x  1234.57")
    });
    case("case-1", || {
        // Names of exactly 10 characters, 11 characters, and none at all.
        check_eq(receipt_line("Strawberry", 1, 0.0), "Strawberry   1 x     0.00")?;
        check_eq(receipt_line("Watermelons", 3, 0.1), "Watermelon   3 x     0.10")?;
        check_eq(receipt_line("", 0, 7.0), "             0 x     7.00")
    });
    case("case-2", || {
        // A quantity or a price wider than its field is printed in full; rounding can carry.
        check_eq(receipt_line("Tea", 1000, 19.999), "Tea        1000 x    20.00")?;
        check_eq(receipt_line("Bun", 4294967295, 12345.6), "Bun        4294967295 x 12345.60")?;
        check_eq(receipt_line("Cake", 999, 99999.999), "Cake       999 x 100000.00")
    });
}
