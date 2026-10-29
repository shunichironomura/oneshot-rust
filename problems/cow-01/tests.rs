use std::borrow::Cow;

use crate::judge::{case, check, check_eq};
use crate::user::clamp_all;

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
    case("example-1", || {
        let data = [1, 5, 3];
        let mut v = Cow::Borrowed(&data[..]);
        clamp_all(&mut v, 9);
        is_borrowed(&v, &[1, 5, 3])?;
        clamp_all(&mut v, 4);
        is_owned(&v, &[1, 4, 3])?;
        check_eq(data, [1, 5, 3])
    });
    case("case-1", || {
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
}
