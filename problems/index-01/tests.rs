use std::ops::Bound;
use std::panic::{self, AssertUnwindSafe};

use crate::judge::{case, check, check_eq};
use crate::user::Grid;

/// Whether `f` panics.
fn panics<R>(f: impl FnOnce() -> R) -> bool {
    panic::catch_unwind(AssertUnwindSafe(f)).is_err()
}

/// A grid with a single row holding `values`.
fn one_row(values: &[i32]) -> Grid<i32> {
    let mut g = Grid::new(values.len(), 1, 0);
    for (x, &v) in values.iter().enumerate() {
        g[(x, 0)] = v;
    }
    g
}

pub fn run() {
    case("sample-1", || {
        let mut g = Grid::new(3, 2, 0);
        g[(2, 1)] = 5;
        g[(0, 0)] += 7;
        check_eq(g[(0, 0)], 7)?;
        check_eq(g.get(2, 1), Some(&5))?;
        check_eq(g.row(1), [0, 0, 5])?;
        check_eq(g.row_sum(1, ..), Some(5))?;
        check_eq(g.row_sum(1, ..2), Some(0))
    });
    case("sample-2", || {
        let g = one_row(&[10, 20, 30, 40]);
        check_eq(g.row_sum(0, 1..3), Some(50))?;
        check_eq(g.row_sum(0, ..=2), Some(60))?;
        check_eq(g.row_sum(0, 2..), Some(70))?;
        check_eq(g.row_sum(0, (Bound::Excluded(0), Bound::Included(2))), Some(50))?;
        check_eq(g.row_sum(0, 3..5), None)
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
        // Empty selections, ranges past the width, start > end, rows out of range.
        let g = one_row(&[1, 2, 3]);
        check_eq(g.row_sum(0, 1..1), Some(0))?;
        check_eq(g.row_sum(0, 3..3), Some(0))?;
        check_eq(g.row_sum(0, 3..), Some(0))?;
        check_eq(g.row_sum(0, ..0), Some(0))?;
        check_eq(g.row_sum(0, 2..=1), Some(0))?;
        check_eq(g.row_sum(0, (Bound::Excluded(1), Bound::Excluded(2))), Some(0))?;
        check_eq(g.row_sum(0, (Bound::Excluded(0), Bound::Unbounded)), Some(5))?;
        check_eq(g.row_sum(0, ..=3), None)?;
        check_eq(g.row_sum(0, 0..4), None)?;
        check_eq(g.row_sum(0, 4..), None)?;
        check_eq(g.row_sum(0, 4..4), None)?;
        check_eq(g.row_sum(0, (Bound::Excluded(3), Bound::Included(3))), None)?;
        check_eq(g.row_sum(0, 2..1), None)?;
        check_eq(g.row_sum(0, (Bound::Excluded(2), Bound::Excluded(2))), None)?;
        check_eq(g.row_sum(1, ..), None)?;
        check_eq(g.row_sum(usize::MAX, 0..1), None)
    });
    case("hidden-5", || {
        // Bounds at usize::MAX are past every grid; computing them must not overflow.
        let g = one_row(&[1, 2, 3]);
        check_eq(g.row_sum(0, ..=usize::MAX), None)?;
        check_eq(g.row_sum(0, 1..=usize::MAX), None)?;
        check_eq(g.row_sum(0, (Bound::Excluded(usize::MAX), Bound::Unbounded)), None)?;
        check_eq(g.row_sum(0, (Bound::Excluded(usize::MAX), Bound::Included(usize::MAX))), None)?;
        check_eq(g.row_sum(0, usize::MAX..), None)
    });
    case("hidden-6", || {
        // The sum may not fit in an i32.
        let g = one_row(&[i32::MAX, i32::MAX, i32::MAX]);
        check_eq(g.row_sum(0, ..), Some(3 * i64::from(i32::MAX)))?;
        let g = one_row(&[i32::MIN, i32::MIN, 5, i32::MAX]);
        check_eq(g.row_sum(0, ..2), Some(2 * i64::from(i32::MIN)))?;
        check_eq(g.row_sum(0, 1..), Some(i64::from(i32::MIN) + 5 + i64::from(i32::MAX)))
    });
    case("hidden-7", || {
        // Grids with no columns or no cells at all.
        let g = Grid::new(0, 2, 1);
        check_eq(g.row(1).len(), 0)?;
        check(panics(|| g.row(2)), "row(2) should panic on a grid of height 2, even with width 0")?;
        check_eq(g.get(0, 0), None)?;
        check_eq(g.row_sum(1, ..), Some(0))?;
        check_eq(g.row_sum(1, ..=0), None)?;
        check_eq(g.row_sum(2, ..), None)?;
        let e = Grid::new(0, 0, 1);
        check_eq(e.get(0, 0), None)?;
        check_eq(e.row_sum(0, ..), None)
    });
}
