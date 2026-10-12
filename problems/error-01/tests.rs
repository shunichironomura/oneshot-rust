use std::collections::HashMap;

use crate::judge::{case, check_eq};
use crate::user::{ConfigError, parse_config};

fn map(xs: &[(&str, u32)]) -> HashMap<String, u32> {
    xs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
}

fn int_err(s: &str) -> std::num::ParseIntError {
    s.parse::<u32>().unwrap_err()
}

pub fn run() {
    case("sample-1", || {
        check_eq(
            parse_config("port = 8080\nworkers=4\n"),
            Ok(map(&[("port", 8080), ("workers", 4)])),
        )
    });
    case("sample-2", || {
        let err = parse_config("port = 8080\nverbose\n").unwrap_err();
        check_eq(&err, &ConfigError::MissingEquals { line: 2 })?;
        check_eq(err.to_string(), "line 2: missing '='")
    });
    case("hidden-1", || check_eq(parse_config(""), Ok(HashMap::new())));
    case("hidden-2", || {
        let err = parse_config("a = 1\nb = two\n").unwrap_err();
        check_eq(&err, &ConfigError::BadNumber(int_err("two")))?;
        check_eq(err.to_string(), "bad number: invalid digit found in string")?;
        let err = parse_config("a =\n").unwrap_err();
        check_eq(&err, &ConfigError::BadNumber(int_err("")))?;
        check_eq(err.to_string(), "bad number: cannot parse integer from empty string")
    });
    case("hidden-3", || {
        let err = parse_config("x = 1\ny = 2\nx = 3\n").unwrap_err();
        check_eq(&err, &ConfigError::Duplicate("x".to_string()))?;
        check_eq(err.to_string(), "duplicate key: x")
    });
    case("hidden-4", || {
        // The first error wins, and a line is split at its first '='.
        check_eq(parse_config("a = 1\nk = v\nnope\n"), Err(ConfigError::BadNumber(int_err("v"))))?;
        check_eq(parse_config("k = 1 = 2\n"), Err(ConfigError::BadNumber(int_err("1 = 2"))))?;
        check_eq(parse_config("big = 4294967296\n"), Err(ConfigError::BadNumber(int_err("4294967296"))))
    });
}
