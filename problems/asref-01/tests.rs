use crate::judge::{case, check_eq};
use crate::user::join_upper;

pub fn run() {
    case("example-1", || {
        check_eq(join_upper(&["ab", "c"], "-"), "AB-C")?;
        check_eq(join_upper(&vec![String::from("é")], ","), "É")?;
        check_eq(join_upper::<String>(&[], ","), "")
    });
    case("case-1", || {
        let names = vec![String::from("straße"), String::new(), String::from("x")];
        check_eq(join_upper(&names, ", "), "STRASSE, , X")?;
        check_eq(join_upper(&names[2..], "+"), "X")?;
        check_eq(join_upper(&["a", "b", "c"], ""), "ABC")?;
        check_eq(join_upper::<&str>(&[], "-"), "")
    });
}
