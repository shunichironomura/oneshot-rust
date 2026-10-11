use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Missing(&'static str),
    BadPort { value: String, source: ParseIntError },
    OutOfRange(u32),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ConfigError {
    // Make `source()` return the cause of a `BadPort` error here.
}

pub fn parse_port(s: &str) -> Result<u16, ConfigError> {
    todo!()
}

pub fn load(pairs: &[(&str, &str)]) -> Result<(String, u16, u64), Box<dyn Error>> {
    todo!()
}

pub fn root_cause<'a>(e: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static) {
    todo!()
}

pub fn report(e: &dyn Error) -> String {
    todo!()
}
