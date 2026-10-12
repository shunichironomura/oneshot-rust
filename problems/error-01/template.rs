use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingEquals { line: usize },
    BadNumber(ParseIntError),
    Duplicate(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub fn parse_config(text: &str) -> Result<HashMap<String, u32>, ConfigError> {
    todo!()
}
