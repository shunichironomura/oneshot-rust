use crate::judge::{case, check_eq};
use crate::user::Collatz;

pub fn run() {
    case("example-1", || check_eq(Collatz::new(6).collect::<Vec<_>>(), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]));
    case("example-2", || check_eq(Collatz::new(1).collect::<Vec<_>>(), vec![1]));
    case("case-1", || check_eq(Collatz::new(0).next(), None));
    case("case-2", || {
        // Once finished, it stays finished.
        let mut it = Collatz::new(2);
        let got = [it.next(), it.next(), it.next(), it.next()];
        check_eq(got, [Some(2), Some(1), None, None])
    });
    case("case-3", || {
        let c = Collatz::new(27);
        check_eq(c.count(), 112)?;
        check_eq(Collatz::new(27).max(), Some(9232))
    });
    case("case-4", || {
        check_eq(Collatz::new(7).take(4).sum::<u64>(), 7 + 22 + 11 + 34)?;
        check_eq(Collatz::new(3).filter(|n| n % 2 == 1).collect::<Vec<_>>(), vec![3, 5, 1])
    });
    case("case-5", || {
        // Lazy: only the requested items are computed.
        check_eq(Collatz::new(u64::MAX / 4).nth(1), Some(3 * (u64::MAX / 4) + 1))
    });
}
