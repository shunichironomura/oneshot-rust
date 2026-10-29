use crate::judge::{case, check_eq};
use crate::user::{finish_order, squares_round_robin};

pub fn run() {
    case("example-1", || check_eq(squares_round_robin(4), [0, 1, 4, 9]));
    case("example-2", || check_eq(finish_order(&[2, 0, 1]), [1, 2, 0]));
    case("case-1", || {
        check_eq(squares_round_robin(0), Vec::<u32>::new())?;
        check_eq(squares_round_robin(1), [0])
    });
    case("case-2", || check_eq(squares_round_robin(5), [0, 1, 4, 9, 16]));
    case("case-3", || {
        check_eq(finish_order(&[]), Vec::<usize>::new())?;
        check_eq(finish_order(&[4]), [0])
    });
    case("case-4", || {
        // Tasks that finish in the same round are listed in index order.
        check_eq(finish_order(&[0, 0, 0]), [0, 1, 2])?;
        check_eq(finish_order(&[2, 1, 2, 0, 1]), [3, 1, 4, 0, 2])
    });
    case("case-5", || {
        check_eq(finish_order(&[3, 2, 1, 0]), [3, 2, 1, 0])?;
        check_eq(finish_order(&[1, 3, 1, 0, 3, 2]), [3, 0, 2, 5, 1, 4])
    });
}
