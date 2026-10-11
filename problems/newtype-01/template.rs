use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// A valid username, such as `alice_1`.
#[derive(Debug, Clone)]
pub struct Username(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameError {
    TooShort,
    TooLong,
    BadStart,
    BadChar(char),
}

impl Username {
    pub fn parse(s: &str) -> Result<Username, NameError> {
        todo!()
    }
}

impl Deref for Username {
    type Target = str;

    fn deref(&self) -> &str {
        todo!()
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        todo!()
    }
}

impl Borrow<str> for Username {
    fn borrow(&self) -> &str {
        todo!()
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl PartialEq<&str> for Username {
    fn eq(&self, other: &&str) -> bool {
        todo!()
    }
}

impl PartialEq for Username {
    fn eq(&self, other: &Username) -> bool {
        todo!()
    }
}

impl Eq for Username {}

impl PartialOrd for Username {
    fn partial_cmp(&self, other: &Username) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for Username {
    fn cmp(&self, other: &Username) -> Ordering {
        todo!()
    }
}

impl Hash for Username {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}
