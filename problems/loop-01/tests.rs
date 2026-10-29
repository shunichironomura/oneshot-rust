use crate::judge::{case, check_eq};
use crate::user::{classify, find_cell, parse_setting};

pub fn run() {
    case("example-1", || {
        let grid = [vec![1, 2], vec![3, 2]];
        check_eq(find_cell(&grid, 2), Some((0, 1)))?;
        check_eq(find_cell(&grid, 4), None)
    });
    case("example-2", || {
        check_eq(parse_setting(" width = 80 "), Some(("width", 80)))?;
        check_eq(parse_setting("height"), None)
    });
    case("example-3", || {
        check_eq(classify(&[3, 0, 5]), "zero")?;
        check_eq(classify(&[4, -2]), "negative")
    });
    case("case-1", || {
        // Row by row: the end of the first row comes before the start of the second.
        check_eq(find_cell(&[vec![0, 0, 7], vec![7, 0]], 7), Some((0, 2)))?;
        check_eq(find_cell(&[vec![5]], 5), Some((0, 0)))?;
        check_eq(find_cell(&[vec![1, 1], vec![1, 9]], 9), Some((1, 1)))
    });
    case("case-2", || {
        // Rows of different lengths, empty rows and an empty grid.
        let grid = [vec![], vec![4, -1], vec![], vec![8, 6, -1], vec![6]];
        check_eq(find_cell(&grid, 6), Some((3, 1)))?;
        check_eq(find_cell(&grid, -1), Some((1, 1)))?;
        check_eq(find_cell(&grid, 0), None)?;
        check_eq(find_cell(&[], 0), None)?;
        check_eq(find_cell(&[vec![], vec![]], 0), None)
    });
    case("case-3", || {
        // The key and the value are trimmed, and the key may contain spaces.
        check_eq(parse_setting("depth=3"), Some(("depth", 3)))?;
        check_eq(parse_setting("\tmax width\t=\t0\t"), Some(("max width", 0)))?;
        check_eq(parse_setting("big = 4294967295"), Some(("big", u32::MAX)))
    });
    case("case-4", || {
        // No '=' at all, or an empty key.
        check_eq(parse_setting(""), None)?;
        check_eq(parse_setting("width 80"), None)?;
        check_eq(parse_setting("= 80"), None)?;
        check_eq(parse_setting(" \t = 80"), None)
    });
    case("case-5", || {
        // The value must parse as a u32, and the line is split at its first '='.
        check_eq(parse_setting("width ="), None)?;
        check_eq(parse_setting("width = -1"), None)?;
        check_eq(parse_setting("width = eighty"), None)?;
        check_eq(parse_setting("width = 4294967296"), None)?;
        check_eq(parse_setting("width = 8 0"), None)?;
        check_eq(parse_setting("a = b = 1"), None)?;
        check_eq(parse_setting("n = 1 = 2"), None)
    });
    case("case-6", || {
        check_eq(classify(&[]), "empty")?;
        check_eq(classify(&[0]), "zero")?;
        check_eq(classify(&[7]), "positive")?;
        check_eq(classify(&[1, i32::MAX, 2]), "positive")
    });
    case("case-7", || {
        // A negative element wins over a zero, wherever they are.
        check_eq(classify(&[0, -1]), "negative")?;
        check_eq(classify(&[5, 0, 0, i32::MIN]), "negative")?;
        check_eq(classify(&[-3]), "negative")
    });
}
