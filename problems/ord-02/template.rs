use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
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
