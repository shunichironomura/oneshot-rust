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

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
