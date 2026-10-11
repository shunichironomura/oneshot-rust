use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// A valid username, such as `alice_1`.
///
/// The derived `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash` only look at the `String`, and
/// those of `String` are the ones of `str`. That is exactly what `Borrow<str>` requires.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        let name = s.trim();
        let len = name.chars().count();
        if len < 3 {
            return Err(NameError::TooShort);
        }
        if len > 16 {
            return Err(NameError::TooLong);
        }
        let mut chars = name.chars();
        if !matches!(chars.next(), Some('a'..='z')) {
            return Err(NameError::BadStart);
        }
        match chars.find(|c| !matches!(c, 'a'..='z' | '0'..='9' | '_')) {
            Some(c) => Err(NameError::BadChar(c)),
            None => Ok(Username(name.to_string())),
        }
    }
}

impl Deref for Username {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Username {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl PartialEq<&str> for Username {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
