use crate::judge::{case, check_eq};
use crate::user::Describe;

/// A type of the harness, with its own `Describe` impl.
struct Celsius(i32);

impl Describe for Celsius {
    fn describe(&self) -> String {
        format!("{}°C", self.0)
    }
}

pub fn run() {
    case("sample-1", || {
        check_eq("hi".describe(), "\"hi\"")?;
        check_eq(vec![Some(1), None].describe(), "[some(1), none]")?;
        check_eq(vec![&5, &6].describe(), "[5, 6]")
    });
    case("sample-2", || {
        check_eq((true, vec!["a", "b"]).describe(), "(yes, [\"a\", \"b\"])")?;
        check_eq(Some(String::from("x")).describe(), "some(\"x\")")
    });
    case("hidden-1", || {
        check_eq(i32::MIN.describe(), "-2147483648")?;
        check_eq(false.describe(), "no")?;
        check_eq(Vec::<i32>::new().describe(), "[]")?;
        check_eq(None::<bool>.describe(), "none")
    });
    case("hidden-2", || {
        check_eq(vec![vec![], vec![1]].describe(), "[[], [1]]")?;
        check_eq((1, (false, None::<i32>)).describe(), "(1, (no, none))")
    });
    case("hidden-3", || {
        let v = vec![10, 20, 30];
        let flags = [true, false];
        check_eq(v[2..].describe(), "[30]")?;
        check_eq(vec![&v[..1], &v[3..]].describe(), "[[10], []]")?;
        check_eq(Some(&flags[1..]).describe(), "some([no])")
    });
    case("hidden-4", || {
        let word = String::from("café");
        check_eq((&&&7).describe(), "7")?;
        check_eq(Some(word.as_str()).describe(), "some(\"café\")")?;
        check_eq((&"a", vec![&word]).describe(), "(\"a\", [\"café\"])")
    });
    case("hidden-5", || {
        // Strings are escaped the way `{:?}` does it.
        check_eq("say \"hi\"\n".describe(), "\"say \\\"hi\\\"\\n\"")?;
        check_eq("a\\b".describe(), "\"a\\\\b\"")?;
        check_eq(String::new().describe(), "\"\"")
    });
    case("hidden-6", || {
        // Values of different types in one `Vec`.
        let list = vec![true, false];
        let nothing: Option<i32> = None;
        let warm = Celsius(21);
        let items: Vec<&dyn Describe> = vec![&1, &"two", &list, &nothing, &warm];
        check_eq(items.describe(), "[1, \"two\", [yes, no], none, 21°C]")
    });
    case("hidden-7", || {
        let today = Celsius(-4);
        check_eq(vec![Celsius(0), Celsius(35)].describe(), "[0°C, 35°C]")?;
        check_eq((Some(&today), &today).describe(), "(some(-4°C), -4°C)")
    });
}
