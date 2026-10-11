use crate::judge::{case, check_eq};
use crate::user::{bits, hex_color, receipt_line, table_row};

pub fn run() {
    case("sample-1", || {
        check_eq(receipt_line("Coffee", 2, 3.5), "Coffee       2 x     3.50")?;
        check_eq(receipt_line("Chocolate bar", 12, 1234.567), "Chocolate   12 x  1234.57")
    });
    case("sample-2", || {
        check_eq(hex_color(128, 255, 90), "#80FF5A")?;
        check_eq(bits(5), "0b00000101")
    });
    case("sample-3", || check_eq(table_row(&["a", "bb"], 4), "| a  | bb |"));
    case("hidden-1", || {
        // Names of exactly 10 characters, 11 characters, and none at all.
        check_eq(receipt_line("Strawberry", 1, 0.0), "Strawberry   1 x     0.00")?;
        check_eq(receipt_line("Watermelons", 3, 0.1), "Watermelon   3 x     0.10")?;
        check_eq(receipt_line("", 0, 7.0), "             0 x     7.00")
    });
    case("hidden-2", || {
        // A quantity or a price wider than its field is printed in full; rounding can carry.
        check_eq(receipt_line("Tea", 1000, 19.999), "Tea        1000 x    20.00")?;
        check_eq(receipt_line("Bun", 4294967295, 12345.6), "Bun        4294967295 x 12345.60")?;
        check_eq(receipt_line("Cake", 999, 99999.999), "Cake       999 x 100000.00")
    });
    case("hidden-3", || {
        // Every channel keeps its leading zero and uses uppercase digits.
        check_eq(hex_color(0, 3, 254), "#0003FE")?;
        check_eq(hex_color(10, 171, 12), "#0AAB0C")?;
        check_eq(hex_color(0, 0, 0), "#000000")
    });
    case("hidden-4", || {
        check_eq(bits(0), "0b00000000")?;
        check_eq(bits(128), "0b10000000")?;
        check_eq(bits(255), "0b11111111")
    });
    case("hidden-5", || {
        // Odd padding puts the extra space on the right; long cells are not cut.
        check_eq(table_row(&["ab", "", "abcde"], 5), "| ab  |     |abcde|")?;
        check_eq(table_row(&["toolong"], 3), "|toolong|")
    });
    case("hidden-6", || {
        // No cells, and a width of 0.
        check_eq(table_row(&[], 3), "|")?;
        check_eq(table_row(&[], 0), "|")?;
        check_eq(table_row(&["a", "bc"], 0), "|a|bc|")
    });
}
