use crate::judge::{case, check, check_eq};

// No `use` for the macro or for `HashMap`: the macro must work by path alone.
type Map<K, V> = std::collections::HashMap<K, V>;

pub fn run() {
    case("example-1", || {
        let m: Map<&str, i32> = crate::user::hashmap! { "a" => 1, "b" => 2, };
        check_eq(m.len(), 2)?;
        check_eq(m["a"], 1)?;
        check_eq(m["b"], 2)
    });
    case("case-1", || {
        // Empty maps take their types from the context; the last value of a key wins.
        let empty: Map<String, Vec<u8>> = crate::user::hashmap! {};
        check(empty.is_empty(), "hashmap! {} should be empty")?;
        let one: Map<char, u8> = crate::user::hashmap! { 'z' => 26 };
        check_eq(one.len(), 1)?;
        check_eq(one[&'z'], 26)?;
        let m: Map<String, i32> = crate::user::hashmap! {
            String::from("x") => 1,
            "y".to_string() => 10 * 2,
            String::from("x") => 3,
        };
        check_eq(m.len(), 2)?;
        check_eq(m["x"], 3)?;
        check_eq(m["y"], 20)
    });
    case("case-2", || {
        // Every key and value is evaluated once, in the order k1, v1, k2, v2, ...
        let mut log: Vec<&str> = Vec::new();
        let m: Map<u8, char> = crate::user::hashmap! {
            { log.push("k1"); 1 } => { log.push("v1"); 'a' },
            { log.push("k2"); 2 } => { log.push("v2"); 'b' },
            { log.push("k3"); 1 } => { log.push("v3"); 'c' }
        };
        check_eq(log, vec!["k1", "v1", "k2", "v2", "k3", "v3"])?;
        check_eq(m.len(), 2)?;
        check_eq(m[&1], 'c')?;
        check_eq(m[&2], 'b')
    });
}
