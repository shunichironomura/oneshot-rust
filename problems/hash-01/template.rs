use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A tag such as `Rust`. Tags that differ only in ASCII case are equal.
#[derive(Debug, Clone)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        todo!()
    }

    pub fn as_str(&self) -> &str {
        todo!()
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Tag {}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub fn unique_tags(words: &[&str]) -> Vec<Tag> {
    todo!()
}

pub fn tag_counts(words: &[&str]) -> HashMap<Tag, usize> {
    todo!()
}
