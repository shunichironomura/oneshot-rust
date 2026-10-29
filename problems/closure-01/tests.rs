use crate::judge::{case, check_eq};
use crate::user::{compose, make_counter};

pub fn run() {
    case("example-1", || {
        let mut c = make_counter(3);
        check_eq([c(), c(), c()], [3, 6, 9])
    });
    case("example-2", || {
        let h = compose(|x: i32| x + 1, |x: i32| x * 10);
        check_eq(h(4), 50)
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
    case("case-4", || {
        // Types may change from step to step, and captured values must stay alive.
        let suffix = String::from("!");
        let shout = compose(|s: &str| s.to_uppercase(), move |s: String| s + &suffix);
        let len = compose(|s: &str| s.len(), |n: usize| n * 2);
        check_eq((shout("hey"), shout("ok"), len("abc")), ("HEY!".to_string(), "OK!".to_string(), 6))
    });
    case("case-5", || {
        let inc = |x: u64| x + 1;
        let twice = compose(inc, inc);
        let four = compose(&twice, &twice);
        check_eq((twice(0), four(10)), (2, 14))
    });
}
