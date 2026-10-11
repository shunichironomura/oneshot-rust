use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingEquals { line: usize },
    EmptyKey { line: usize },
    BadNumber(ParseIntError),
    Duplicate(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingEquals { line } => write!(f, "line {line}: missing '='"),
            ConfigError::EmptyKey { line } => write!(f, "line {line}: empty key"),
            ConfigError::BadNumber(e) => write!(f, "bad number: {e}"),
            ConfigError::Duplicate(key) => write!(f, "duplicate key: {key}"),
        }
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

pub fn parse_config(text: &str) -> Result<HashMap<String, u32>, ConfigError> {
    let mut map = HashMap::new();
    for (i, raw) in text.lines().enumerate() {
        let line = i + 1;
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (key, value) = trimmed
            .split_once('=')
            .ok_or(ConfigError::MissingEquals { line })?;
        let key = key.trim();
        if key.is_empty() {
            return Err(ConfigError::EmptyKey { line });
        }
        let value: u32 = value.trim().parse()?;
        if map.insert(key.to_string(), value).is_some() {
            return Err(ConfigError::Duplicate(key.to_string()));
        }
    }
    Ok(map)
}
