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

fn parse_number(s: &str) -> Result<u64, ParseVersionError> {
    // `parse::<u64>()` alone would also accept a leading `+`.
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ParseVersionError);
    }
    // Only digits are left, so the only possible error is an overflow.
    s.parse().map_err(|_| ParseVersionError)
}

fn is_valid_pre(pre: &str) -> bool {
    !pre.is_empty() && pre.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'.')
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The label may contain dots, so split it off before splitting at `.`.
        let (numbers, pre) = match s.split_once('-') {
            Some((numbers, pre)) if is_valid_pre(pre) => (numbers, Some(pre.to_string())),
            Some(_) => return Err(ParseVersionError),
            None => (s, None),
        };
        let parts = numbers
            .split('.')
            .map(parse_number)
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
