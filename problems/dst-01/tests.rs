use std::fmt::Debug;
use std::rc::Rc;

use crate::judge::{case, check, check_eq};
use crate::user::{byte_size, share, show_pair};

// A type of the harness, with no `derive` and no `impl`.
struct Rgb(u8, u8, u8);

pub fn run() {
    case("example-1", || {
        check_eq(byte_size("héllo"), 6)?;
        check_eq(byte_size(&[1u16, 2, 3][..]), 6)?;
        check_eq(byte_size(&7u64), 8)
    });
    case("example-2", || {
        check_eq(show_pair("x", &[1, 2][..]), "\"x\" & [1, 2]")?;
        check_eq(show_pair(&5, "y"), "5 & \"y\"")
    });
    case("example-3", || {
        let s: Rc<str> = share(Box::<str>::from("abc"));
        let v: Rc<[i32]> = share(vec![1, 2].into_boxed_slice());
        let n: Rc<i32> = share(Box::new(5));
        check_eq(&*s, "abc")?;
        check_eq(&*v, [1, 2])?;
        check_eq(*n, 5)
    });
    case("case-1", || {
        check_eq(byte_size(""), 0)?;
        check_eq(byte_size("日本"), 6)?;
        let words = String::from("Rust 🦀");
        check_eq(byte_size(&words[5..]), 4)?;
        let v = vec![1u32, 2, 3];
        check_eq(byte_size(&v[..]), 12)?;
        check_eq(byte_size(&v[..0]), 0)?;
        check_eq(byte_size(&[0u8; 5]), 5)
    });
    case("case-2", || {
        // For a trait object, the size of the value behind it.
        let d: &dyn Debug = &1u32;
        check_eq(byte_size(d), 4)?;
        let flags: &dyn Debug = &[true; 3];
        check_eq(byte_size(flags), 3)?;
        let pixels = vec![Rgb(0, 0, 0), Rgb(255, 128, 0)];
        check_eq(byte_size(&pixels[1]), 3)?;
        check_eq(byte_size(&pixels[..]), 6)
    });
    case("case-3", || {
        check_eq(show_pair("a", "b"), "\"a\" & \"b\"")?;
        let name = String::from("Ferris");
        check_eq(show_pair(&name[..4], &[0u8; 0][..]), "\"Ferr\" & []")?;
        let d: &dyn Debug = &Some('c');
        check_eq(show_pair(d, &-1.5), "Some('c') & -1.5")
    });
    case("case-4", || {
        let empty: Rc<[u8]> = share(Vec::new().into_boxed_slice());
        check(empty.is_empty(), "should be empty")?;
        let s: Rc<str> = share(String::from("héllo").into_boxed_str());
        let t = Rc::clone(&s);
        check_eq(Rc::strong_count(&s), 2)?;
        check_eq(&*t, "héllo")?;
        // The boxed value may borrow a local variable.
        let word = String::from("hi");
        let r: Rc<&str> = share(Box::new(word.as_str()));
        check_eq(*r, "hi")
    });
    case("case-5", || {
        let offset = 10;
        let add: Box<dyn Fn(i32) -> i32> = Box::new(move |x| x + offset);
        let f: Rc<dyn Fn(i32) -> i32> = share(add);
        let g = Rc::clone(&f);
        check_eq(f(1), 11)?;
        check_eq(g(-10), 0)?;
        check_eq(Rc::strong_count(&f), 2)
    });
}
