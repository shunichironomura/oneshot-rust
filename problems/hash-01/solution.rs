use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

/// A tag such as `Rust`. Tags that differ only in ASCII case are equal.
#[derive(Debug, Clone)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for Tag {}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Equal tags must hash equally, so hash exactly what `eq` compares:
        // the bytes with ASCII letters lowercased.
        for b in self.0.bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
        // Like `str`, end with a byte that never occurs in UTF-8, so that the
        // pairs ("ab", "c") and ("a", "bc") feed the hasher different bytes.
        state.write_u8(0xff);
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn unique_tags(words: &[&str]) -> Vec<Tag> {
    let mut seen = HashSet::new();
    let mut tags = Vec::new();
    for &word in words {
        let tag = Tag::new(word);
        // `insert` returns `false` (and keeps the stored tag) if an equal tag is already there.
        if seen.insert(tag.clone()) {
            tags.push(tag);
        }
    }
    tags
}

pub fn tag_counts(words: &[&str]) -> HashMap<Tag, usize> {
    let mut counts = HashMap::new();
    for &word in words {
        // When the tag is already a key, `entry` drops the new `Tag` and keeps the first one.
        *counts.entry(Tag::new(word)).or_insert(0) += 1;
    }
    counts
}
