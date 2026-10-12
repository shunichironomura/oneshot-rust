use std::borrow::Cow;

use crate::judge::{case, check, check_eq};
use crate::user::{Doc, clamp_all, normalize_ws};

fn borrowed(got: Cow<'_, str>, expected: &str) -> Result<(), String> {
    match got {
        Cow::Borrowed(s) => check_eq(s, expected),
        Cow::Owned(s) => Err(format!("expected Borrowed({expected:?}), got Owned({s:?})")),
    }
}

fn owned(got: Cow<'_, str>, expected: &str) -> Result<(), String> {
    match got {
        Cow::Owned(s) => check_eq(s.as_str(), expected),
        Cow::Borrowed(s) => Err(format!("expected Owned({expected:?}), got Borrowed({s:?})")),
    }
}

fn is_borrowed(v: &Cow<'_, [i32]>, expected: &[i32]) -> Result<(), String> {
    match v {
        Cow::Borrowed(s) => check_eq(*s, expected),
        Cow::Owned(o) => Err(format!("expected Borrowed({expected:?}), got Owned({o:?})")),
    }
}

fn is_owned(v: &Cow<'_, [i32]>, expected: &[i32]) -> Result<(), String> {
    match v {
        Cow::Owned(o) => check_eq(o.as_slice(), expected),
        Cow::Borrowed(s) => Err(format!("expected Owned({expected:?}), got Borrowed({s:?})")),
    }
}

pub fn run() {
    case("sample-1", || {
        borrowed(normalize_ws("a b"), "a b")?;
        borrowed(normalize_ws("  hi\n"), "hi")?;
        owned(normalize_ws("a  b\tc "), "a b c")
    });
    case("sample-2", || {
        let data = [1, 5, 3];
        let mut v = Cow::Borrowed(&data[..]);
        clamp_all(&mut v, 9);
        is_borrowed(&v, &[1, 5, 3])?;
        clamp_all(&mut v, 4);
        is_owned(&v, &[1, 4, 3])?;
        check_eq(data, [1, 5, 3])
    });
    case("sample-3", || {
        let a = Doc::new(&["x", "y"]);
        let mut b = a.clone();
        check(a.shares_storage(&b), "a clone should share the storage")?;
        b.set_line(0, "z");
        check(!a.shares_storage(&b), "after set_line, b should have its own storage")?;
        check_eq((a.line(0), b.line(0)), (Some("x"), Some("z")))?;
        check_eq((a.line(1), b.line(1)), (Some("y"), Some("y")))
    });
    case("hidden-1", || {
        // Every kind of ASCII whitespace, runs of plain spaces; trimming alone borrows a slice
        // of the input.
        borrowed(normalize_ws(""), "")?;
        borrowed(normalize_ws(" \t\r\n\x0C"), "")?;
        owned(normalize_ws("a\tb"), "a b")?;
        owned(normalize_ws(" a  b   c "), "a b c")?;
        owned(normalize_ws("\tx \r\n y\x0Cz  "), "x y z")?;
        let s = String::from("\n  one two three \t");
        match normalize_ws(&s) {
            Cow::Borrowed(t) => {
                check_eq(t, "one two three")?;
                check(t.as_ptr() == s[3..].as_ptr(), "the result should be a slice of the input")
            }
            Cow::Owned(t) => Err(format!("expected Borrowed, got Owned({t:?})")),
        }
    });
    case("hidden-2", || {
        // Equal to max is not greater; empty and negative values.
        let data = [4, -2, 4];
        let mut v = Cow::Borrowed(&data[..]);
        clamp_all(&mut v, 4);
        is_borrowed(&v, &[4, -2, 4])?;
        let mut e: Cow<'_, [i32]> = Cow::Borrowed(&[]);
        clamp_all(&mut e, -100);
        is_borrowed(&e, &[])?;
        let neg = [-1, -5, -3, 0];
        let mut n = Cow::Borrowed(&neg[..]);
        clamp_all(&mut n, -3);
        is_owned(&n, &[-3, -5, -3, -3])?;
        check_eq(neg, [-1, -5, -3, 0])?;
        // An Owned value stays Owned and is changed in place.
        let mut v: Cow<'_, [i32]> = Cow::Owned(vec![10, 2, 7]);
        let buf = v.as_ptr();
        clamp_all(&mut v, 5);
        is_owned(&v, &[5, 2, 5])?;
        check(v.as_ptr() == buf, "an Owned Vec should be changed in place")?;
        clamp_all(&mut v, 6);
        is_owned(&v, &[5, 2, 5])?;
        let data = [9, 8];
        let mut w = Cow::Borrowed(&data[..]);
        clamp_all(&mut w, 8);
        is_owned(&w, &[8, 8])?;
        let buf = w.as_ptr();
        clamp_all(&mut w, 3);
        is_owned(&w, &[3, 3])?;
        check(w.as_ptr() == buf, "an Owned Vec should be changed in place")
    });
    case("hidden-3", || {
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
    case("hidden-4", || {
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
    case("hidden-5", || {
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
