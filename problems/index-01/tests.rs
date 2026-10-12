use std::panic::{self, AssertUnwindSafe};

use crate::judge::{case, check, check_eq};
use crate::user::Grid;

/// Whether `f` panics.
fn panics<R>(f: impl FnOnce() -> R) -> bool {
    panic::catch_unwind(AssertUnwindSafe(f)).is_err()
}

pub fn run() {
    case("sample-1", || {
        let mut g = Grid::new(3, 2, 0);
        g[(2, 1)] = 5;
        g[(0, 0)] += 7;
        check_eq(g[(0, 0)], 7)?;
        check_eq(g.get(2, 1), Some(&5))?;
        check_eq(g.row(1), [0, 0, 5])
    });
    case("hidden-1", || {
        // Each coordinate is checked on its own, not only the flattened index.
        let mut g = Grid::new(3, 2, 0);
        g[(0, 1)] = 4;
        check_eq(g.get(3, 0), None)?;
        check_eq(g.get(0, 2), None)?;
        check_eq(g.get(0, 1), Some(&4))?;
        check_eq(g.get(usize::MAX, 1), None)?;
        check_eq(g.get(1, usize::MAX), None)?;
        check_eq(g.get(usize::MAX, usize::MAX), None)?;
        check(panics(|| g[(3, 0)]), "g[(3, 0)] should panic on a 3x2 grid")?;
        check(panics(|| g[(3, 0)] = 9), "g[(3, 0)] = 9 should panic on a 3x2 grid")?;
        check_eq(g.row(0), [0, 0, 0])?;
        check_eq(g.row(1), [4, 0, 0])
    });
    case("hidden-2", || {
        // A grid that is taller than it is wide: x is the column, y is the row.
        let mut g = Grid::new(2, 3, 'a');
        g[(1, 2)] = 'z';
        g[(0, 1)] = 'm';
        check_eq(g.row(0), ['a', 'a'])?;
        check_eq(g.row(1), ['m', 'a'])?;
        check_eq(g.row(2), ['a', 'z'])?;
        check_eq(g.get(1, 2), Some(&'z'))?;
        check_eq(g.get(2, 1), None)?;
        check_eq(g[(0, 1)], 'm')
    });
    case("hidden-3", || {
        // Cells that are not Copy: indexing hands out references.
        let mut g = Grid::new(2, 2, String::from("-"));
        g[(1, 0)].push_str("hi");
        g[(0, 1)] = String::from("x");
        check_eq(g.row(0), ["-", "-hi"])?;
        check_eq(g.row(1), ["x", "-"])?;
        check_eq(g.get(1, 0).map(String::as_str), Some("-hi"))?;
        check_eq(g[(1, 1)].as_str(), "-")
    });
    case("hidden-4", || {
        // Grids with no columns or no cells at all.
        let g = Grid::new(0, 2, 1);
        check_eq(g.row(1).len(), 0)?;
        check_eq(g.get(0, 0), None)?;
        check(panics(|| g[(0, 1)]), "g[(0, 1)] should panic on a grid of width 0")?;
        let e = Grid::new(0, 0, 1);
        check_eq(e.get(0, 0), None)
    });
}
