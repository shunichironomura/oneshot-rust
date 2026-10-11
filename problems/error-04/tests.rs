use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

use crate::judge::{case, check, check_eq};
use crate::user::{ConfigError, load, parse_port, report, root_cause};

type Loaded = Result<(String, u16, u64), Box<dyn Error>>;

fn u32_err(s: &str) -> ParseIntError {
    s.parse::<u32>().unwrap_err()
}

fn u64_err(s: &str) -> ParseIntError {
    s.parse::<u64>().unwrap_err()
}

fn bad_port(s: &str) -> ConfigError {
    ConfigError::BadPort { value: s.to_string(), source: u32_err(s) }
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
    case("sample-1", || {
        expect_ok(load(&[("host", "h"), ("port", "8080")]), "h", 8080, 30)?;
        check_eq(parse_port("70000"), Err(ConfigError::OutOfRange(70000)))
    });
    case("sample-2", || {
        let e = expect_config_err(load(&[("host", "h"), ("port", "x1")]), bad_port("x1"))?;
        check_eq(e.to_string(), "bad port 'x1'")?;
        let root = root_cause(&*e);
        check(root.is::<ParseIntError>(), "the root cause should be a ParseIntError")?;
        check_eq(root.to_string(), "invalid digit found in string")?;
        check_eq(report(&*e), "bad port 'x1': invalid digit found in string")
    });
    case("sample-3", || {
        let e = expect_parse_err(
            load(&[("host", "h"), ("port", "80"), ("timeout", "-1")]),
            u64_err("-1"),
        )?;
        check_eq(report(&*e), "invalid digit found in string")
    });
    case("hidden-1", || {
        // Valid ports, and numbers that parse but are not ports.
        check_eq(parse_port("1"), Ok(1))?;
        check_eq(parse_port("65535"), Ok(65535))?;
        check_eq(parse_port("+80"), Ok(80))?;
        check_eq(parse_port("0080"), Ok(80))?;
        check_eq(parse_port("0"), Err(ConfigError::OutOfRange(0)))?;
        check_eq(parse_port("65536"), Err(ConfigError::OutOfRange(65536)))?;
        check_eq(parse_port("4294967295"), Err(ConfigError::OutOfRange(4294967295)))
    });
    case("hidden-2", || {
        // Not a u32: no trimming, empty, negative, too large for a u32.
        check_eq(parse_port(" 80"), Err(bad_port(" 80")))?;
        check_eq(parse_port("80\n"), Err(bad_port("80\n")))?;
        check_eq(parse_port(""), Err(bad_port("")))?;
        check_eq(parse_port("-1"), Err(bad_port("-1")))?;
        check_eq(parse_port("4294967296"), Err(bad_port("4294967296")))
    });
    case("hidden-3", || {
        // Display and source() of each variant.
        let missing = ConfigError::Missing("port");
        check_eq(missing.to_string(), "missing key: port")?;
        check(missing.source().is_none(), "Missing should have no source")?;
        let range = ConfigError::OutOfRange(0);
        check_eq(range.to_string(), "port 0 out of range")?;
        check(range.source().is_none(), "OutOfRange should have no source")?;
        let bad = bad_port("");
        check_eq(bad.to_string(), "bad port ''")?;
        let Some(cause) = bad.source() else {
            return Err("BadPort should have a source".to_string());
        };
        check_eq(cause.downcast_ref::<ParseIntError>(), Some(&u32_err("")))
    });
    case("hidden-4", || {
        // Missing keys, checked in the order host, port, timeout.
        expect_config_err(load(&[]), ConfigError::Missing("host"))?;
        expect_config_err(load(&[("port", "80")]), ConfigError::Missing("host"))?;
        expect_config_err(load(&[("port", "x"), ("timeout", "y")]), ConfigError::Missing("host"))?;
        expect_config_err(
            load(&[("Host", "h"), (" host", "h"), ("port", "80")]),
            ConfigError::Missing("host"),
        )?;
        expect_config_err(load(&[("host", "h")]), ConfigError::Missing("port"))?;
        let e = expect_config_err(
            load(&[("host", "h"), ("timeout", "x")]),
            ConfigError::Missing("port"),
        )?;
        check_eq(e.to_string(), "missing key: port")
    });
    case("hidden-5", || {
        // The first pair of a key wins, other keys are ignored, and an empty host is fine.
        expect_ok(
            load(&[
                ("port", "1"),
                ("user", "root"),
                ("host", ""),
                ("host", "b"),
                ("port", "2"),
                ("timeout", "0"),
            ]),
            "",
            1,
            0,
        )?;
        expect_config_err(load(&[("host", "h"), ("port", "x"), ("port", "80")]), bad_port("x"))?;
        expect_ok(
            load(&[("timeout", "5"), ("host", "a b"), ("port", "65535"), ("timeout", "x")]),
            "a b",
            65535,
            5,
        )
    });
    case("hidden-6", || {
        // Port errors are boxed ConfigErrors, and they come before timeout errors.
        expect_config_err(
            load(&[("host", "h"), ("timeout", "x"), ("port", "99999")]),
            ConfigError::OutOfRange(99999),
        )?;
        expect_config_err(load(&[("host", "h"), ("port", ""), ("timeout", "x")]), bad_port(""))?;
        let range = ConfigError::OutOfRange(0);
        let e = expect_config_err(load(&[("host", "h"), ("port", "0")]), range)?;
        check_eq(root_cause(&*e).downcast_ref::<ConfigError>(), Some(&ConfigError::OutOfRange(0)))?;
        check_eq(report(&*e), "port 0 out of range")
    });
    case("hidden-7", || {
        // A bad timeout is the ParseIntError of the u64 itself.
        let pairs = |t| [("host", "h"), ("port", "80"), ("timeout", t)];
        expect_parse_err(load(&pairs(" 5")), u64_err(" 5"))?;
        expect_parse_err(load(&pairs("")), u64_err(""))?;
        let huge = "18446744073709551616";
        let e = expect_parse_err(load(&pairs(huge)), u64_err(huge))?;
        check(root_cause(&*e).is::<ParseIntError>(), "a ParseIntError is its own root cause")?;
        expect_ok(load(&pairs("18446744073709551615")), "h", 80, u64::MAX)
    });
    case("hidden-8", || {
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
        check_eq(root_cause(&outer).downcast_ref::<ParseIntError>(), Some(&u32_err("x1")))?;
        let inner = Box::new(ConfigError::OutOfRange(65536));
        let outer = Context { what: "a", source: Box::new(Context { what: "b", source: inner }) };
        check_eq(
            root_cause(&outer).downcast_ref::<ConfigError>(),
            Some(&ConfigError::OutOfRange(65536)),
        )?;
        let lone = u64_err("");
        let root = root_cause(&lone);
        check(std::ptr::addr_eq(root, &lone), "an error without a source is its own root cause")?;
        check_eq(report(&lone), "cannot parse integer from empty string")
    });
}
