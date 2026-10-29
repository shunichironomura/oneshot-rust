use std::sync::Barrier;
use std::thread;

use crate::judge::{case, check_eq};
use crate::user::{count, distinct, register, total};

// All cases share the program-wide counters and run in this order. `sample-1` starts from zero;
// the other cases compare `total()` and `distinct()` with their values at the start of the case.
// Every type is registered in one case only, apart from the repeated names in `hidden-1`, which
// are different types.
pub fn run() {
    case("example-1", || {
        struct Apple;
        struct Pear;
        struct Plum;
        check_eq(register::<Apple>(), 1)?;
        check_eq(register::<Apple>(), 2)?;
        check_eq(register::<Pear>(), 1)?;
        check_eq(count::<Apple>(), 2)?;
        check_eq(count::<Plum>(), 0)?;
        check_eq((total(), distinct()), (3, 2))
    });
    case("example-2", || {
        type Bytes = Vec<u8>;
        let (t, d) = (total(), distinct());
        check_eq(register::<Vec<u8>>(), 1)?;
        check_eq(register::<Bytes>(), 2)?;
        check_eq(register::<Vec<u16>>(), 1)?;
        check_eq((total(), distinct()), (t + 3, d + 2))
    });
    case("case-1", || {
        // Structs with the same name declared in different places are different types, although
        // `std::any::type_name` gives them the same name.
        struct Apple;
        let (t, d) = (total(), distinct());
        check_eq(register::<Apple>(), 1)?;
        let (first, count_first) = {
            struct Tag;
            (register::<Tag>(), || count::<Tag>())
        };
        let second = {
            struct Tag;
            (count::<Tag>(), register::<Tag>())
        };
        check_eq((first, second, count_first()), (1, (0, 1), 1))?;
        check_eq((total(), distinct()), (t + 3, d + 3))
    });
    case("case-2", || {
        // `count` never registers a type, and does not change any counter.
        struct Ghost;
        struct Real;
        let (t, d) = (total(), distinct());
        check_eq((count::<Ghost>(), count::<Real>()), (0, 0))?;
        check_eq((total(), distinct()), (t, d))?;
        check_eq(register::<Real>(), 1)?;
        check_eq((count::<Real>(), count::<Ghost>()), (1, 0))?;
        check_eq((total(), distinct()), (t + 1, d + 1))
    });
    case("case-3", || {
        // Another thread registers at the same time as this one, and the threads share the
        // counters.
        struct Shared;
        struct Solo;
        let (t, d) = (total(), distinct());
        check_eq(register::<Shared>(), 1)?;
        let start = Barrier::new(2);
        let (mine, (solo, theirs)) = thread::scope(|s| {
            let worker = s.spawn(|| {
                let solo = register::<Solo>();
                start.wait();
                (solo, [(); 4].map(|_| register::<Shared>()))
            });
            start.wait();
            let mine = [(); 4].map(|_| register::<Shared>());
            (mine, worker.join().unwrap())
        });
        let mut got = [mine, theirs].concat();
        got.sort();
        check_eq(got, [2, 3, 4, 5, 6, 7, 8, 9])?;
        check_eq((solo, count::<Solo>(), count::<Shared>()), (1, 1, 9))?;
        check_eq((total(), distinct()), (t + 10, d + 2))
    });
}
