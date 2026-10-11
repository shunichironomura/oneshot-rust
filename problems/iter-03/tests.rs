use crate::judge::{case, check_eq};
use crate::user::{longest, powers_of, running_balance, split_header};

pub fn run() {
    case("sample-1", || {
        check_eq(longest(&["hi", "hello", "world"]), Some("hello"))?;
        check_eq(longest(&["éé", "abc"]), Some("abc"))
    });
    case("sample-2", || {
        check_eq(
            split_header(&["# a", "# b", "body", "# x"]),
            (vec!["# a", "# b"], vec!["body", "# x"]),
        )
    });
    case("sample-3", || {
        check_eq(running_balance(100, &[-30, 50, -200, 10]), vec![70, 120])?;
        check_eq(powers_of(10).len(), 20)?;
        check_eq(powers_of(2).last(), Some(&(1 << 63)))
    });
    case("hidden-1", || {
        // Characters, not bytes; the first of equally long words wins.
        check_eq(longest(&["ab", "日本"]), Some("ab"))?;
        check_eq(longest(&["x", "abc", "日本語", "def"]), Some("abc"))?;
        check_eq(longest(&["🦀", "ab"]), Some("ab"))?;
        check_eq(longest(&["", "é", "a"]), Some("é"))?;
        check_eq(longest(&["solo"]), Some("solo"))?;
        check_eq(longest(&[""]), Some(""))?;
        check_eq(longest(&[]), None)
    });
    case("hidden-2", || {
        // Only headers, no header, nothing at all.
        let none: Vec<&str> = Vec::new();
        check_eq(split_header(&["#a", "#b"]), (vec!["#a", "#b"], none.clone()))?;
        check_eq(split_header(&["x", "#y", "z"]), (none.clone(), vec!["x", "#y", "z"]))?;
        check_eq(split_header(&[]), (none.clone(), none))
    });
    case("hidden-3", || {
        // An empty line or a leading space ends the header.
        check_eq(split_header(&["#", "#a", "", "#b"]), (vec!["#", "#a"], vec!["", "#b"]))?;
        check_eq(split_header(&["#t", " #a", "#b"]), (vec!["#t"], vec![" #a", "#b"]))
    });
    case("hidden-4", || {
        check_eq(running_balance(0, &[-1]), Vec::<i64>::new())?;
        check_eq(running_balance(5, &[]), Vec::<i64>::new())?;
        check_eq(running_balance(50, &[-51, 100]), Vec::<i64>::new())?;
        // The rejected transaction is not output, and nothing after it is applied.
        check_eq(running_balance(20, &[5, -30, 10, 1]), vec![25])?;
        // A balance of exactly 0 is fine.
        check_eq(running_balance(10, &[-10, 0, 3, -3, -1, 100]), vec![0, 0, 3, 0])?;
        check_eq(running_balance(0, &[0, 7, -7, 2]), vec![0, 7, 0, 2])?;
        check_eq(running_balance(1, &[-1, -1, 5]), vec![0])
    });
    case("hidden-5", || {
        let twos = powers_of(2);
        check_eq(twos.len(), 64)?;
        check_eq((twos[0], twos[1], twos[2], twos[63]), (1, 2, 4, 1 << 63))?;
        let threes = powers_of(3);
        check_eq(threes.len(), 41)?;
        check_eq((threes[0], threes[1], threes[2], threes[40]), (1, 3, 9, 12157665459056928801))?;
        check_eq(powers_of(10).last(), Some(&10_000_000_000_000_000_000))
    });
    case("hidden-6", || {
        // The largest power fits exactly or only just.
        check_eq(powers_of(u64::MAX), vec![1, u64::MAX])?;
        check_eq(powers_of(1 << 63), vec![1, 1 << 63])?;
        check_eq(powers_of(1 << 32), vec![1, 1 << 32])?;
        check_eq(powers_of(4294967295), vec![1, 4294967295, 18446744065119617025])?;
        // Powers of two whose next power would be exactly 2^64.
        check_eq(powers_of(65536), vec![1, 1 << 16, 1 << 32, 1 << 48])?;
        check_eq(powers_of(256).len(), 8)?;
        check_eq(powers_of(16).len(), 16)
    });
}
