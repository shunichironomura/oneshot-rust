use crate::judge::{case, check_eq};
use crate::user::{char_len, join_upper, square_in_place, total_chars};

// A type of the harness that lends out its `u32` field.
struct Score {
    points: u32,
}

impl AsMut<u32> for Score {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.points
    }
}

pub fn run() {
    case("sample-1", || {
        check_eq(char_len("Café"), 4)?;
        check_eq(char_len(String::from("ab")), 2)?;
        check_eq(char_len(&String::from("x")), 1)?;
        check_eq(join_upper(&["ab", "c"], "-"), "AB-C")?;
        check_eq(join_upper(&vec![String::from("é")], ","), "É")?;
        check_eq(join_upper::<String>(&[], ","), "")
    });
    case("sample-2", || {
        check_eq(total_chars(vec![vec!["ab", "c"], vec![]]), 3)?;
        check_eq(total_chars([["a", "b"]; 3]), 6)?;
        check_eq(total_chars(&[vec![String::from("é")]]), 1)?;
        check_eq(total_chars("a b\nc".lines().map(|l| l.split(' '))), 3)
    });
    case("sample-3", || {
        let mut b = Box::new(3u32);
        square_in_place(&mut b);
        check_eq(*b, 9)?;
        let mut score = Score { points: 12 };
        square_in_place(&mut score);
        check_eq(score.points, 144)
    });
    case("hidden-1", || {
        let owned = String::from("日本語");
        check_eq(char_len(""), 0)?;
        check_eq(char_len(&owned), 3)?;
        check_eq(char_len(owned), 3)?;
        // "e" followed by a combining accent: two `char`s.
        check_eq(char_len(String::from("e\u{301}")), 2)
    });
    case("hidden-2", || {
        let names = vec![String::from("straße"), String::new(), String::from("x")];
        check_eq(join_upper(&names, ", "), "STRASSE, , X")?;
        check_eq(join_upper(&names[2..], "+"), "X")?;
        check_eq(join_upper(&["a", "b", "c"], ""), "ABC")?;
        check_eq(join_upper::<&str>(&[], "-"), "")
    });
    case("hidden-3", || {
        check_eq(total_chars(Vec::<Vec<&str>>::new()), 0)?;
        check_eq(total_chars([Vec::<String>::new(), Vec::new()]), 0)?;
        let groups = vec![vec![String::from("Rust"), String::new()], vec![String::from("🦀")]];
        check_eq(total_chars(&groups), 5)?;
        check_eq(total_chars(groups), 5)?;
        check_eq(total_chars("x,yz;;w".split(';').map(|p| p.split(','))), 4)
    });
    case("hidden-4", || {
        let mut zero = Box::new(0u32);
        square_in_place(&mut zero);
        check_eq(*zero, 0)?;
        let mut big = Box::new(65535u32);
        square_in_place(&mut big);
        check_eq(*big, 4_294_836_225)?;
        let mut score = Score { points: 2 };
        square_in_place(&mut score);
        square_in_place(&mut score);
        check_eq(score.points, 16)
    });
}
