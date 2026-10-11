use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Bound;

use crate::judge::{case, check, check_eq};
use crate::user::{NameError, Username};

/// Parses a name that the test expects to be valid.
fn user(s: &str) -> Username {
    Username::parse(s).unwrap()
}

/// The result of `parse`, with the name copied out through `Deref`.
fn parsed(s: &str) -> Result<String, NameError> {
    Username::parse(s).map(|u| String::from(&*u))
}

fn ok(name: &str) -> Result<String, NameError> {
    Ok(String::from(name))
}

fn char_count(s: &str) -> usize {
    s.chars().count()
}

fn byte_len<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

fn borrowed<B: Borrow<str>>(b: &B) -> &str {
    b.borrow()
}

pub fn run() {
    case("sample-1", || {
        let u = Username::parse("  alice_1 ").unwrap();
        check_eq(&*u, "alice_1")?;
        check_eq(u.len(), 7)?;
        check(u.starts_with("al"), "u.starts_with(\"al\") should be true")?;
        check(u == "alice_1", "u == \"alice_1\" should be true")?;
        check_eq(format!("<{u}>"), "<alice_1>")
    });
    case("sample-2", || {
        check_eq(Username::parse("Al"), Err(NameError::TooShort))?;
        check_eq(Username::parse("9lives"), Err(NameError::BadStart))?;
        check_eq(Username::parse("bad-name"), Err(NameError::BadChar('-')))
    });
    case("sample-3", || {
        let mut set = HashSet::new();
        set.insert(Username::parse("alice").unwrap());
        set.insert(Username::parse("bob").unwrap());
        check(set.contains("bob"), "the set should contain \"bob\"")
    });
    case("hidden-1", || {
        // Lengths are counted in chars after trimming, and both 3 and 16 are allowed.
        check_eq(parsed("abc"), ok("abc"))?;
        check_eq(parsed("abcdefghijklmnop"), ok("abcdefghijklmnop"))?;
        check_eq(Username::parse("abcdefghijklmnopq"), Err(NameError::TooLong))?;
        check_eq(Username::parse("ab"), Err(NameError::TooShort))?;
        check_eq(Username::parse(""), Err(NameError::TooShort))?;
        check_eq(Username::parse("  ab  "), Err(NameError::TooShort))?;
        // 2 chars (3 bytes), and 16 chars (17 bytes).
        check_eq(Username::parse("aé"), Err(NameError::TooShort))?;
        check_eq(Username::parse("abcdefghijklmnoé"), Err(NameError::BadChar('é')))
    });
    case("hidden-2", || {
        // The first character is checked after the length and before the other characters.
        check_eq(Username::parse("Alice"), Err(NameError::BadStart))?;
        check_eq(Username::parse("_alice"), Err(NameError::BadStart))?;
        check_eq(Username::parse("1-2-3"), Err(NameError::BadStart))?;
        check_eq(Username::parse("éclair"), Err(NameError::BadStart))?;
        check_eq(Username::parse("A"), Err(NameError::TooShort))?;
        check_eq(Username::parse("A-very-long-username"), Err(NameError::TooLong))?;
        check_eq(parsed("z__"), ok("z__"))
    });
    case("hidden-3", || {
        // The first character that is not a-z, 0-9 or '_' is reported.
        check_eq(Username::parse("aBc"), Err(NameError::BadChar('B')))?;
        check_eq(Username::parse("a.b-c"), Err(NameError::BadChar('.')))?;
        check_eq(Username::parse("garçon"), Err(NameError::BadChar('ç')))?;
        check_eq(Username::parse("ab\u{663}"), Err(NameError::BadChar('\u{663}')))?;
        check_eq(parsed("a_1_b_2_c_3"), ok("a_1_b_2_c_3"))?;
        check_eq(parsed("user0123456789_z"), ok("user0123456789_z"))
    });
    case("hidden-4", || {
        // Whitespace at both ends is removed first, as by `str::trim`.
        let u = user("\talice\n");
        check_eq(&*u, "alice")?;
        check_eq(u.len(), 5)?;
        check_eq(u.to_string(), "alice")?;
        check_eq(parsed("\u{3000}bob\u{3000}"), ok("bob"))?;
        check_eq(parsed(" \r\n carol_9 \t "), ok("carol_9"))?;
        check_eq(Username::parse("  Bob  "), Err(NameError::BadStart))?;
        check_eq(Username::parse(" a b "), Err(NameError::BadChar(' ')))
    });
    case("hidden-5", || {
        // A username can be used like a `&str`.
        let u = user("dave_42");
        check_eq(char_count(&u), 7)?;
        let s: &str = &u;
        check_eq(s, "dave_42")?;
        check_eq(&u[1..4], "ave")?;
        check(u.ends_with("42") && u.contains('_'), "str methods should work on a username")?;
        check_eq(byte_len(&u), 7)?;
        check_eq(byte_len(u.clone()), 7)?;
        check_eq(borrowed(&u), "dave_42")?;
        check_eq(format!("[{}]", u), "[dave_42]")?;
        check_eq([user("ann"), u].join(", "), "ann, dave_42")
    });
    case("hidden-6", || {
        // `==` with a `&str` compares the exact name; usernames compare by name.
        let u = user("alice");
        check(u == "alice", "u == \"alice\" should be true")?;
        check(u != "Alice", "u != \"Alice\" should be true")?;
        check(u != " alice", "u != \" alice\" should be true")?;
        check(u != "alic", "u != \"alic\" should be true")?;
        check(u != "alice_", "u != \"alice_\" should be true")?;
        check(u == user(" alice "), "equal names should give equal usernames")?;
        check(u != user("alicia"), "different names should give different usernames")?;
        check(u.clone() == u, "a clone should be equal")
    });
    case("hidden-7", || {
        // Hash maps and sets can be queried with a `&str`.
        let names = ["alice", "bob", "carol_9", "dave"];
        let mut ages: HashMap<Username, u32> = names.into_iter().map(user).zip(20..).collect();
        for name in names {
            if !ages.contains_key(name) {
                return Err(format!("the map should contain {name:?}"));
            }
        }
        check(
            !ages.contains_key("Alice") && !ages.contains_key(" bob") && !ages.contains_key("dav"),
            "the map should not contain other strings",
        )?;
        check_eq(ages.get("bob"), Some(&21))?;
        if let Some(age) = ages.get_mut("carol_9") {
            *age += 10;
        }
        check_eq(ages.get("carol_9"), Some(&32))?;
        check_eq(ages.remove("dave"), Some(23))?;
        check(!ages.contains_key("dave"), "\"dave\" should be removed")?;
        check_eq(ages.len(), 3)?;
        let set: HashSet<Username> = ages.into_keys().collect();
        check(set.contains("alice") && set.contains("carol_9"), "the set should contain the names")?;
        check_eq(set.get("bob").map(|u| &**u), Some("bob"))
    });
    case("hidden-8", || {
        // Usernames are ordered like their names, and B-tree maps can be queried with a `&str`.
        let (al_x, alice, bob) = (user("al_x"), user("alice"), user("bob"));
        check(al_x < alice, "al_x should be less than alice")?;
        check(bob > al_x, "bob should be greater than al_x")?;
        check_eq(bob.cmp(&alice), Ordering::Greater)?;
        check_eq(alice.partial_cmp(&alice.clone()), Some(Ordering::Equal))?;
        let names = ["bob", "alice", "al_x", "a1b", "b0b", "carol"];
        let mut map = BTreeMap::new();
        for (i, name) in names.into_iter().enumerate() {
            map.insert(user(name), i);
        }
        let keys: Vec<&str> = map.keys().map(|u| &**u).collect();
        check_eq(keys, ["a1b", "al_x", "alice", "b0b", "bob", "carol"])?;
        for (i, name) in names.into_iter().enumerate() {
            if map.get(name) != Some(&i) {
                return Err(format!("map.get({name:?}) should be Some({i})"));
            }
        }
        check_eq(map.get("ab"), None)?;
        let from_b: Vec<&str> = map
            .range::<str, _>((Bound::Included("b"), Bound::Unbounded))
            .map(|(k, _)| &**k)
            .collect();
        check_eq(from_b, ["b0b", "bob", "carol"])
    });
}
