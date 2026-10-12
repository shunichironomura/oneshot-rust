use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    /// The pre-release label, such as `rc.1` in `2.0.0-rc.1`.
    pub pre: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVersionError;

// Already implemented: parses `MAJOR[.MINOR[.PATCH]][-PRE]`.
impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (numbers, pre) = match s.split_once('-') {
            Some((numbers, pre)) => (numbers, Some(pre.to_string())),
            None => (s, None),
        };
        let parts = numbers
            .split('.')
            .map(|part| part.parse().map_err(|_| ParseVersionError))
            .collect::<Result<Vec<u64>, _>>()?;
        let (major, minor, patch) = match parts[..] {
            [major] => (major, 0, 0),
            [major, minor] => (major, minor, 0),
            [major, minor, patch] => (major, minor, patch),
            _ => return Err(ParseVersionError),
        };
        Ok(Version { major, minor, patch, pre })
    }
}

// Already implemented: prints `MAJOR.MINOR.PATCH`, followed by `-PRE` if there is one.
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre {
            write!(f, "-{pre}")?;
        }
        Ok(())
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

pub fn latest_stable(versions: &[Version]) -> Option<&Version> {
    todo!()
}
