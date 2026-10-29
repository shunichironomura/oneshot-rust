use crate::judge::{case, check_eq};
use crate::user::find_cell;

pub fn run() {
    case("example-1", || {
        let grid = [vec![1, 2], vec![3, 2]];
        check_eq(find_cell(&grid, 2), Some((0, 1)))?;
        check_eq(find_cell(&grid, 4), None)
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
}
