use crate::judge::{case, check, check_eq};
use crate::user::Doc;

pub fn run() {
    case("example-1", || {
        let a = Doc::new(&["x", "y"]);
        let mut b = a.clone();
        check(a.shares_storage(&b), "a clone should share the storage")?;
        b.set_line(0, "z");
        check(!a.shares_storage(&b), "after set_line, b should have its own storage")?;
        check_eq((a.line(0), b.line(0)), (Some("x"), Some("z")))?;
        check_eq((a.line(1), b.line(1)), (Some("y"), Some("y")))
    });
    case("case-1", || {
        // Edits on a Doc that does not share its storage, including after a snapshot is dropped.
        let mut d = Doc::new(&["a", "b", "c"]);
        d.set_line(1, "B");
        d.set_line(2, "C");
        d.set_line(1, "bb");
        check_eq([d.line(0), d.line(1), d.line(2), d.line(3)], [Some("a"), Some("bb"), Some("C"), None])?;
        check_eq(Doc::new(&[]).line(0), None)?;
        let a = Doc::new(&["x", "y"]);
        let mut b = a.clone();
        drop(a);
        b.set_line(1, "w");
        check_eq((b.line(0), b.line(1)), (Some("x"), Some("w")))
    });
    case("case-2", || {
        // Equal lines are not the same storage.
        let a = Doc::new(&["same"]);
        let b = Doc::new(&["same"]);
        check(!a.shares_storage(&b), "two new Docs should not share storage")?;
        let c = b.clone();
        check(b.shares_storage(&c) && c.shares_storage(&b), "a clone should share the storage")?;
        check(a.shares_storage(&a), "a Doc shares storage with itself")?;
        let e1 = Doc::new(&[]);
        let e2 = Doc::new(&[]);
        check(!e1.shares_storage(&e2), "two new empty Docs should not share storage")
    });
    case("case-3", || {
        // Three snapshots: an edit only ever affects the edited one.
        let mut a = Doc::new(&["1", "2"]);
        let mut b = a.clone();
        let mut c = b.clone();
        c.set_line(1, "c");
        check(a.shares_storage(&b), "a and b should still share")?;
        check(!c.shares_storage(&a) && !c.shares_storage(&b), "c should have its own storage")?;
        c.set_line(0, "c0");
        b.set_line(0, "b");
        check(!a.shares_storage(&b), "b should have its own storage")?;
        a.set_line(0, "a");
        check_eq([a.line(0), a.line(1)], [Some("a"), Some("2")])?;
        check_eq([b.line(0), b.line(1)], [Some("b"), Some("2")])?;
        check_eq([c.line(0), c.line(1)], [Some("c0"), Some("c")])
    });
}
