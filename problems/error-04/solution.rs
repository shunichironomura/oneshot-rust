use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Missing(&'static str),
    BadPort { value: String, source: ParseIntError },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing(key) => write!(f, "missing key: {key}"),
            // The cause is not repeated here: callers reach it through `source()`.
            ConfigError::BadPort { value, .. } => write!(f, "bad port '{value}'"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::BadPort { source, .. } => Some(source),
            ConfigError::Missing(_) => None,
        }
    }
}

pub fn parse_port(s: &str) -> Result<u16, ConfigError> {
    s.parse().map_err(|source| ConfigError::BadPort {
        value: s.to_string(),
        source,
    })
}

/// The value of the first pair whose key is `key`.
fn get<'a>(pairs: &[(&str, &'a str)], key: &str) -> Option<&'a str> {
    pairs.iter().find(|&&(k, _)| k == key).map(|&(_, v)| v)
}

pub fn load(pairs: &[(&str, &str)]) -> Result<(String, u16, u64), Box<dyn Error>> {
    let host = get(pairs, "host").ok_or(ConfigError::Missing("host"))?;
    let port = get(pairs, "port").ok_or(ConfigError::Missing("port"))?;
    let port = parse_port(port)?;
    let timeout: u64 = match get(pairs, "timeout") {
        Some(t) => t.parse()?,
        None => 30,
    };
    Ok((host.to_string(), port, timeout))
}

pub fn report(e: &dyn Error) -> String {
    let mut out = e.to_string();
    let mut cur = e;
    while let Some(next) = cur.source() {
        out.push_str(": ");
        out.push_str(&next.to_string());
        cur = next;
    }
    out
}
