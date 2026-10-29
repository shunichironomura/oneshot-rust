use crate::judge::{case, check_eq};
use crate::user::make_counter;

pub fn run() {
    case("example-1", || {
        let mut c = make_counter(3);
        check_eq([c(), c(), c()], [3, 6, 9])
    });
    case("case-1", || {
        // Each counter keeps its own state.
        let mut a = make_counter(1);
        let mut b = make_counter(5);
        check_eq([a(), b(), a(), b(), a()], [1, 5, 2, 10, 3])
    });
    case("case-2", || {
        let mut z = make_counter(0);
        check_eq([z(), z()], [0, 0])
    });
    case("case-3", || {
        // The counter may outlive the place where `step` came from.
        let mut counters: Vec<_> = (1..=3).map(make_counter).collect();
        let firsts: Vec<u32> = counters.iter_mut().map(|c| c()).collect();
        let seconds: Vec<u32> = counters.iter_mut().map(|c| c()).collect();
        check_eq((firsts, seconds), (vec![1, 2, 3], vec![2, 4, 6]))
    });
}
