use crate::judge::{case, check, check_eq};
use crate::user::Tree;

fn build(values: &[i32]) -> Tree {
    let mut t = Tree::new();
    for &v in values {
        t.insert(v);
    }
    t
}

pub fn run() {
    case("example-1", || {
        let t = build(&[5, 3, 8, 1, 4]);
        check_eq(t.to_sorted_vec(), vec![1, 3, 4, 5, 8])?;
        check_eq(t.height(), 3)?;
        check(t.contains(4), "contains(4) should be true")?;
        check(!t.contains(7), "contains(7) should be false")
    });
    case("example-2", || {
        let t = Tree::new();
        check_eq(t.to_sorted_vec(), Vec::<i32>::new())?;
        check_eq(t.height(), 0)?;
        check(!t.contains(0), "an empty tree contains nothing")
    });
    case("case-1", || {
        let mut t = Tree::new();
        check_eq(t.insert(2), true)?;
        check_eq(t.insert(1), true)?;
        check_eq(t.insert(2), false)?;
        check_eq(t.insert(1), false)?;
        check_eq(t.to_sorted_vec(), vec![1, 2])?;
        check_eq(t.height(), 2)
    });
    case("case-2", || {
        let values: Vec<i32> = (1..=20).collect();
        let t = build(&values);
        check_eq(t.to_sorted_vec(), values)?;
        check_eq(t.height(), 20)
    });
    case("case-3", || {
        let t = build(&[0, -7, 7, -3, 3, -10, 10]);
        check_eq(t.to_sorted_vec(), vec![-10, -7, -3, 0, 3, 7, 10])?;
        check_eq(t.height(), 3)?;
        for v in [-10, -7, -3, 0, 3, 7, 10] {
            check(t.contains(v), "every inserted value should be found")?;
        }
        for v in [-8, -1, 1, 5, 11] {
            check(!t.contains(v), "a value that was never inserted was found")?;
        }
        Ok(())
    });
    case("case-4", || {
        let t = build(&[42]);
        check_eq(t.height(), 1)?;
        check(t.contains(42), "contains(42) should be true")?;
        check_eq(t.to_sorted_vec(), vec![42])
    });
}
