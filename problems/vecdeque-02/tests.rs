use crate::judge::{case, check_eq};
use crate::user::josephus;

const NOBODY: [usize; 0] = [];

pub fn run() {
    case("example-1", || check_eq(josephus(7, 3), [3, 6, 2, 7, 5, 1, 4]));
    case("case-1", || {
        check_eq(josephus(0, 3), NOBODY)?;
        check_eq(josephus(5, 1), [1, 2, 3, 4, 5])?;
        check_eq(josephus(2, 2), [2, 1])?;
        check_eq(josephus(5, 5), [5, 1, 3, 4, 2])?;
        check_eq(josephus(10, 2), [2, 4, 6, 8, 10, 3, 7, 1, 9, 5])
    });
    case("case-2", || {
        // k larger than the circle.
        check_eq(josephus(3, 10), [1, 3, 2])?;
        check_eq(josephus(9, 4), [4, 8, 3, 9, 6, 5, 7, 2, 1])
    });
    case("case-3", || {
        // Very large k, up to usize::MAX.
        check_eq(josephus(8, 1_000_000_000), [8, 6, 3, 2, 1, 4, 7, 5])?;
        check_eq(josephus(6, usize::MAX), [3, 2, 6, 5, 1, 4])?;
        check_eq(josephus(1, usize::MAX), [1])
    });
}
