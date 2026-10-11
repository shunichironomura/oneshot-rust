use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AppError {
    MissingComma,
    Parse(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingComma => write!(f, "missing comma"),
            AppError::Parse(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

impl Error for AppError {}

/// Lets `?` turn any library error into `AppError::Parse`.
impl<E: Error> From<E> for AppError {
    fn from(e: E) -> Self {
        AppError::Parse(e.to_string())
    }
}

pub fn parse_pair(s: &str) -> Result<(i32, f64), AppError> {
    let (a, b) = s.split_once(',').ok_or(AppError::MissingComma)?;
    Ok((a.trim().parse()?, b.trim().parse()?))
}

pub fn sum_pairs(text: &str) -> Result<f64, Box<dyn Error>> {
    let mut total = 0.0;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (a, b) = parse_pair(line)?;
        total += a as f64 + b;
    }
    Ok(total)
}
