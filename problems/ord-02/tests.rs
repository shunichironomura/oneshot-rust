use crate::judge::{case, check_eq};
use crate::user::{ParseVersionError, Version};

fn ver(major: u64, minor: u64, patch: u64, pre: Option<&str>) -> Version {
    Version { major, minor, patch, pre: pre.map(String::from) }
}

fn all_invalid(inputs: &[&str]) -> Result<(), String> {
    for s in inputs {
        let got = s.parse::<Version>();
        if got != Err(ParseVersionError) {
            return Err(format!("{s:?}: expected Err(ParseVersionError), got {got:?}"));
        }
    }
    Ok(())
}

fn parses_to(s: &str, expected: Version) -> Result<(), String> {
    check_eq(s.parse::<Version>(), Ok(expected)).map_err(|m| format!("{s:?}: {m}"))
}

pub fn run() {
    case("sample-1", || {
        check_eq("1.2".parse::<Version>(), Ok(ver(1, 2, 0, None)))?;
        check_eq("3-rc.1".parse::<Version>(), Ok(ver(3, 0, 0, Some("rc.1"))))?;
        check_eq("1.2.3.4".parse::<Version>(), Err(ParseVersionError))
    });
    case("sample-2", || all_invalid(&["", "1..2", "1.2-", "v1"]));
    case("hidden-1", || {
        // Missing or extra parts, whitespace and other characters.
        all_invalid(&[
            "", "1.", "1..2", "-1", "1.-2", " 1.2", "1.2 ", "v1", "1.0.0+build", "\u{ff11}.2",
        ])
    });
    case("hidden-2", || {
        // Signs, and numbers that do not fit in a u64.
        all_invalid(&["+1", "1.+2", "+0-rc", "18446744073709551616"])
    });
    case("hidden-3", || {
        // Bad pre-release labels.
        all_invalid(&[
            "1.2-", "1.2.3-", "1-beta-2", "1.0.0-rc_1", "1.0.0-rc 1", "1.0.0-+x", "1.0.0-\u{3b2}",
            "1.0.0-caf\u{e9}", "1.0.0-\u{663}",
        ])
    });
    case("hidden-4", || {
        // Valid edge cases.
        parses_to("0", ver(0, 0, 0, None))?;
        parses_to("007.010", ver(7, 10, 0, None))?;
        parses_to("1.18446744073709551615", ver(1, u64::MAX, 0, None))?;
        parses_to("1-alpha", ver(1, 0, 0, Some("alpha")))?;
        parses_to("1.2-rc.1", ver(1, 2, 0, Some("rc.1")))?;
        parses_to("2.0.0-RC.1", ver(2, 0, 0, Some("RC.1")))?;
        parses_to("0010.02.3-007", ver(10, 2, 3, Some("007")))
    });
}
