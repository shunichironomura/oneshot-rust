use crate::judge::{case, check, check_eq};
use crate::user::{Lender, count_all, numbered, windows_mut};

/// Copies every window of `v` into a `Vec`.
fn windows_of(v: &mut [i32], size: usize) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    let mut windows = windows_mut(v, size);
    while let Some(w) = windows.next() {
        out.push(w.to_vec());
    }
    out
}

pub fn run() {
    case("sample-1", || {
        let mut v = vec![1, 2, 3, 4];
        let mut windows = windows_mut(&mut v, 2);
        while let Some(w) = windows.next() {
            w[1] += w[0];
        }
        check_eq(v, [1, 3, 6, 10])
    });
    case("sample-2", || {
        let mut lines = numbered("apple\nbanana");
        check_eq(lines.next(), Some("1: apple"))?;
        check_eq(lines.next(), Some("2: banana"))?;
        check_eq(lines.next(), None)
    });
    case("sample-3", || {
        check_eq(count_all(windows_mut(&mut [0; 5], 3)), 3)?;
        check_eq(count_all(numbered("a\nb\nc")), 3)
    });
    case("hidden-1", || {
        // The windows from left to right, one element apart.
        let mut v = vec![1, 2, 3, 4, 5];
        check_eq(windows_of(&mut v, 3), [vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]])?;
        check_eq(windows_of(&mut v, 5), [[1, 2, 3, 4, 5]])?;
        check_eq(windows_of(&mut v, 6), Vec::<Vec<i32>>::new())?;
        check_eq(windows_of(&mut [], 1), Vec::<Vec<i32>>::new())
    });
    case("hidden-2", || {
        // A change made through one window is seen by the next ones.
        let mut fib = [1, 1, 0, 0, 0, 0, 0, 0];
        let mut windows = windows_mut(&mut fib, 3);
        while let Some(w) = windows.next() {
            w[2] = w[0] + w[1];
        }
        check(windows.next().is_none(), "expected None again after the last window")?;
        check_eq(fib, [1, 1, 2, 3, 5, 8, 13, 21])
    });
    case("hidden-3", || {
        // Elements that are not Copy, and elements that are references.
        let mut words = [String::from("a"), String::from("b"), String::from("c")];
        let mut windows = windows_mut(&mut words, 2);
        while let Some(w) = windows.next() {
            w.swap(0, 1);
        }
        check_eq(words, ["b", "c", "a"])?;
        let mut parts = ["x", "y", "z"];
        let mut windows = windows_mut(&mut parts, 3);
        while let Some(w) = windows.next() {
            w.reverse();
        }
        check_eq(parts, ["z", "y", "x"])
    });
    case("hidden-4", || {
        // Empty text, "\r\n", an empty line, non-ASCII text and a final line ending.
        let mut empty = numbered("");
        check_eq(empty.next(), None)?;
        check_eq(empty.next(), None)?;
        let text = String::from("a\r\n\n日本\n");
        let mut lines = numbered(&text);
        let mut got = Vec::new();
        while let Some(line) = lines.next() {
            got.push(line.to_string());
        }
        check_eq(got, ["1: a", "2: ", "3: 日本"])
    });
    case("hidden-5", || {
        // count_all counts only the items that are left.
        let mut v = [0; 5];
        let mut windows = windows_mut(&mut v, 2);
        windows.next();
        check_eq(count_all(windows), 3)?;
        check_eq(count_all(windows_mut(&mut [0; 5], 6)), 0)?;
        let mut lines = numbered("a\nb\nc");
        lines.next();
        check_eq(count_all(lines), 2)
    });
    case("hidden-6", || {
        // Items of two different lenders can be used at the same time.
        let mut v = vec![1, 2, 3, 10, 20, 30];
        let (left, right) = v.split_at_mut(3);
        let mut a = windows_mut(left, 2);
        let mut b = windows_mut(right, 2);
        while let (Some(x), Some(y)) = (a.next(), b.next()) {
            x[1] += y[0];
            y[1] += x[0];
        }
        check_eq(v, [1, 12, 24, 10, 21, 42])?;
        let mut first = numbered("a\nb");
        let mut second = numbered("c");
        let x = first.next();
        let y = second.next();
        check_eq((x, y), (Some("1: a"), Some("1: c")))
    });
}
