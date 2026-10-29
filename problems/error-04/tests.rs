use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

use crate::judge::{case, check, check_eq};
use crate::user::{ConfigError, load, parse_port, report};

type Loaded = Result<(String, u16, u64), Box<dyn Error>>;

fn u16_err(s: &str) -> ParseIntError {
    s.parse::<u16>().unwrap_err()
}

fn u64_err(s: &str) -> ParseIntError {
    s.parse::<u64>().unwrap_err()
}

fn bad_port(s: &str) -> ConfigError {
    ConfigError::BadPort { value: s.to_string(), source: u16_err(s) }
}

/// An error type of the tests that adds context to another error.
#[derive(Debug)]
struct Context {
    what: &'static str,
    source: Box<dyn Error>,
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.what)
    }
}

impl Error for Context {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.source)
    }
}

/// Expects `Ok((host, port, timeout))`.
fn expect_ok(got: Loaded, host: &str, port: u16, timeout: u64) -> Result<(), String> {
    match got {
        Ok(v) => check_eq(v, (host.to_string(), port, timeout)),
        Err(e) => Err(format!("expected Ok, got Err({e:?})")),
    }
}

/// Expects a boxed `ConfigError` equal to `want`, and returns the box.
fn expect_config_err(got: Loaded, want: ConfigError) -> Result<Box<dyn Error>, String> {
    match got {
        Ok(v) => Err(format!("expected Err({want:?}), got Ok({v:?})")),
        Err(e) => match e.downcast_ref::<ConfigError>() {
            Some(inner) => {
                check_eq(inner, &want)?;
                Ok(e)
            }
            None => Err(format!("expected Err({want:?}), got Err({e:?}), not a ConfigError")),
        },
    }
}

/// Expects a boxed `ParseIntError` equal to `want`, and returns the box.
fn expect_parse_err(got: Loaded, want: ParseIntError) -> Result<Box<dyn Error>, String> {
    match got {
        Ok(v) => Err(format!("expected Err({want:?}), got Ok({v:?})")),
        Err(e) => match e.downcast_ref::<ParseIntError>() {
            Some(inner) => {
                check_eq(inner, &want)?;
                Ok(e)
            }
            None => Err(format!("expected Err({want:?}), got Err({e:?}), not a ParseIntError")),
        },
    }
}

pub fn run() {
    case("example-1", || {
        expect_ok(load(&[("host", "h"), ("port", "8080")]), "h", 8080, 30)?;
        check_eq(parse_port("70000"), Err(bad_port("70000")))
    });
    case("example-2", || {
        let e = expect_config_err(load(&[("host", "h"), ("port", "x1")]), bad_port("x1"))?;
        check_eq(e.to_string(), "bad port 'x1'")?;
        let Some(cause) = e.source() else {
            return Err("BadPort should have a source".to_string());
        };
        check(cause.is::<ParseIntError>(), "the source should be a ParseIntError")?;
        check_eq(cause.to_string(), "invalid digit found in string")?;
        check_eq(report(&*e), "bad port 'x1': invalid digit found in string")
    });
    case("example-3", || {
        let e = expect_parse_err(
            load(&[("host", "h"), ("port", "80"), ("timeout", "-1")]),
            u64_err("-1"),
        )?;
        check_eq(report(&*e), "invalid digit found in string")
    });
    case("case-1", || {
        // Valid ports.
        check_eq(parse_port("1"), Ok(1))?;
        check_eq(parse_port("0"), Ok(0))?;
        check_eq(parse_port("65535"), Ok(65535))?;
        check_eq(parse_port("+80"), Ok(80))?;
        check_eq(parse_port("0080"), Ok(80))
    });
    case("case-2", || {
        // Not a u16: no trimming, empty, negative, too large.
        check_eq(parse_port(" 80"), Err(bad_port(" 80")))?;
        check_eq(parse_port("80\n"), Err(bad_port("80\n")))?;
        check_eq(parse_port(""), Err(bad_port("")))?;
        check_eq(parse_port("-1"), Err(bad_port("-1")))?;
        check_eq(parse_port("65536"), Err(bad_port("65536")))
    });
    case("case-3", || {
        // Display and source() of each variant.
        let missing = ConfigError::Missing("port");
        check_eq(missing.to_string(), "missing key: port")?;
        check(missing.source().is_none(), "Missing should have no source")?;
        let bad = bad_port("");
        check_eq(bad.to_string(), "bad port ''")?;
        let Some(cause) = bad.source() else {
            return Err("BadPort should have a source".to_string());
        };
        check_eq(cause.downcast_ref::<ParseIntError>(), Some(&u16_err("")))
    });
    case("case-4", || {
        // Missing keys, checked in the order host, port, timeout.
        expect_config_err(load(&[]), ConfigError::Missing("host"))?;
        expect_config_err(load(&[("port", "80")]), ConfigError::Missing("host"))?;
        expect_config_err(load(&[("port", "x"), ("timeout", "y")]), ConfigError::Missing("host"))?;
        expect_config_err(load(&[("host", "h")]), ConfigError::Missing("port"))?;
        let e = expect_config_err(
            load(&[("host", "h"), ("timeout", "x")]),
            ConfigError::Missing("port"),
        )?;
        check_eq(e.to_string(), "missing key: port")?;
        check_eq(report(&*e), "missing key: port")
    });
    case("case-5", || {
        // Port errors are boxed ConfigErrors, and they come before timeout errors.
        expect_config_err(
            load(&[("host", "h"), ("timeout", "x"), ("port", "99999")]),
            bad_port("99999"),
        )?;
        let e =
            expect_config_err(load(&[("host", "h"), ("port", ""), ("timeout", "x")]), bad_port(""))?;
        check_eq(report(&*e), "bad port '': cannot parse integer from empty string")?;
        expect_ok(load(&[("timeout", "5"), ("host", "a b"), ("port", "65535")]), "a b", 65535, 5)
    });
    case("case-6", || {
        // A bad timeout is the ParseIntError of the u64 itself.
        let pairs = |t| [("host", "h"), ("port", "80"), ("timeout", t)];
        expect_parse_err(load(&pairs(" 5")), u64_err(" 5"))?;
        expect_parse_err(load(&pairs("")), u64_err(""))?;
        let huge = "18446744073709551616";
        expect_parse_err(load(&pairs(huge)), u64_err(huge))?;
        expect_ok(load(&pairs("18446744073709551615")), "h", 80, u64::MAX)
    });
    case("case-7", || {
        // Chains longer than two errors, made of other error types.
        let inner = load(&[("host", "h"), ("port", "x1")]).unwrap_err();
        let outer = Context {
            what: "cannot start",
            source: Box::new(Context { what: "reading server.conf", source: inner }),
        };
        check_eq(
            report(&outer),
            "cannot start: reading server.conf: bad port 'x1': invalid digit found in string",
        )?;
        let inner = Box::new(ConfigError::Missing("host"));
        let outer = Context { what: "a", source: Box::new(Context { what: "b", source: inner }) };
        check_eq(report(&outer), "a: b: missing key: host")?;
        check_eq(report(&u64_err("")), "cannot parse integer from empty string")
    });
}
