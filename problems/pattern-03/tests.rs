use crate::judge::{case, check_eq};
use crate::user::{initials, rounds_over};

pub fn run() {
    case("example-1", || check_eq(rounds_over(&[3, 7, 5, 9], 5), vec![1, 3]));
    case("example-2", || check_eq(initials(&[('a', 1), ('b', 2)]), "ab"));
    case("case-1", || {
        // Equal to par is not above it.
        check_eq(rounds_over(&[5, 6, 5, 4, 6], 5), vec![1, 4])?;
        check_eq(rounds_over(&[0, 1, 0], 0), vec![1])
    });
    case("case-2", || {
        // No round over par, and no rounds at all.
        check_eq(rounds_over(&[2, 2], 9), Vec::<usize>::new())?;
        check_eq(rounds_over(&[], 0), Vec::<usize>::new())
    });
    case("case-3", || {
        check_eq(initials(&[]), "")?;
        check_eq(initials(&[('日', 3), ('é', 0), ('z', 9)]), "日éz")
    });
}
