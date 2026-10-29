use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::judge::{case, check, check_eq};
use crate::user::{Tag, tag_counts, unique_tags};

/// The spellings of the tags, in order.
fn spellings(tags: &[Tag]) -> Vec<&str> {
    tags.iter().map(Tag::as_str).collect()
}

/// The entries of the map as (spelling, count), sorted by spelling.
fn entries(counts: &HashMap<Tag, usize>) -> Vec<(&str, usize)> {
    let mut v: Vec<(&str, usize)> = counts.iter().map(|(t, &n)| (t.as_str(), n)).collect();
    v.sort();
    v
}

fn hash_of(tag: &Tag) -> u64 {
    let mut h = DefaultHasher::new();
    tag.hash(&mut h);
    h.finish()
}

fn same(a: &str, b: &str) -> Result<(), String> {
    if Tag::new(a) == Tag::new(b) { Ok(()) } else { Err(format!("{a:?} and {b:?} should be equal tags")) }
}

fn different(a: &str, b: &str) -> Result<(), String> {
    if Tag::new(a) != Tag::new(b) { Ok(()) } else { Err(format!("{a:?} and {b:?} should be different tags")) }
}

pub fn run() {
    case("example-1", || check_eq(spellings(&unique_tags(&["Rust", "go", "RUST", "Go"])), vec!["Rust", "go"]));
    case("example-2", || {
        check_eq(entries(&tag_counts(&["Rust", "go", "rust", "RUST"])), vec![("Rust", 3), ("go", 1)])
    });
    case("example-3", || {
        let mut set = HashSet::new();
        check(set.insert(Tag::new("Rust")), "the first insert should return true")?;
        check(set.contains(&Tag::new("rUST")), "the set should contain \"rUST\"")?;
        check(!set.insert(Tag::new("RUST")), "inserting \"RUST\" should return false")?;
        check_eq(set.len(), 1)?;
        check_eq(set.get(&Tag::new("rust")).map(Tag::as_str), Some("Rust"))
    });
    case("case-1", || {
        // Only ASCII letters ignore case; everything else must match exactly.
        same("Rust", "rUST")?;
        same("a-B_c 1", "A-b_C 1")?;
        same("Café", "CAFé")?;
        same("", "")?;
        different("Café", "CAFÉ")?;
        different("É", "é")?;
        different("Straße", "STRASSE")?;
        different("rust", "rusty")?;
        different(" a", "a")
    });
    case("case-2", || {
        // Equal tags must have equal hashes.
        for (a, b) in [("Rust", "rUST"), ("a-B_c 1", "A-b_C 1"), ("Café", "CAFé")] {
            if hash_of(&Tag::new(a)) != hash_of(&Tag::new(b)) {
                return Err(format!("{a:?} and {b:?} hash differently"));
            }
        }
        Ok(())
    });
    case("case-3", || {
        check_eq(spellings(&unique_tags(&["é", "É", "e", "E", "é"])), vec!["é", "É", "e"])?;
        check_eq(entries(&tag_counts(&["Straße", "STRASSE", "strasse"])), vec![("STRASSE", 2), ("Straße", 1)])
    });
    case("case-4", || {
        check_eq(spellings(&unique_tags(&[])), Vec::<&str>::new())?;
        check(tag_counts(&[]).is_empty(), "tag_counts(&[]) should be empty")?;
        check_eq(entries(&tag_counts(&["", "x", "", "X"])), vec![("", 2), ("x", 2)])
    });
    case("case-5", || {
        let tag = Tag::new("RuSt");
        check_eq(tag.as_str(), "RuSt")?;
        check_eq(tag.to_string(), "RuSt")?;
        check_eq(tag.clone().as_str(), "RuSt")?;
        check_eq(Tag::new(" Go ").as_str(), " Go ")
    });
    case("case-6", || {
        let words = ["Apple", "banana", "APPLE", "Cherry", "bAnAnA", "apple"];
        check_eq(spellings(&unique_tags(&words)), vec!["Apple", "banana", "Cherry"])?;
        let counts = tag_counts(&words);
        check_eq(entries(&counts), vec![("Apple", 3), ("Cherry", 1), ("banana", 2)])?;
        check_eq(counts.get(&Tag::new("BANANA")), Some(&2))?;
        check_eq(counts.get(&Tag::new("date")), None)
    });
}
