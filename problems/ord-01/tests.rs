use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};

use crate::judge::{case, check, check_eq};
use crate::user::{ParseVersionError, Version, latest_stable};

fn v(s: &str) -> Version {
    s.parse().unwrap()
}

fn ver(major: u64, minor: u64, patch: u64, pre: Option<&str>) -> Version {
    Version { major, minor, patch, pre: pre.map(String::from) }
}

fn ascending(list: &[Version]) -> Result<(), String> {
    for pair in list.windows(2) {
        if !(pair[0] < pair[1]) {
            return Err(format!("expected {:?} < {:?}", pair[0], pair[1]));
        }
    }
    Ok(())
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
        let short = "1.2".parse::<Version>();
        check_eq(&short, &Ok(ver(1, 2, 0, None)))?;
        check_eq("3-rc.1".parse::<Version>(), Ok(ver(3, 0, 0, Some("rc.1"))))?;
        check_eq("1.2.3.4".parse::<Version>(), Err(ParseVersionError))?;
        check_eq(short.unwrap().to_string(), "1.2.0")?;
        check_eq(ver(1, 0, 0, Some("beta")).to_string(), "1.0.0-beta")
    });
    case("sample-2", || {
        ascending(&[ver(1, 9, 0, None), ver(1, 10, 0, None)])?;
        ascending(&[
            ver(1, 0, 0, Some("alpha")),
            ver(1, 0, 0, Some("alpha.1")),
            ver(1, 0, 0, Some("beta")),
            ver(1, 0, 0, None),
            ver(1, 0, 1, None),
        ])?;
        let set: BTreeSet<Version> = ["1.2", "1.2.0", "1.2.0-rc1"].into_iter().map(v).collect();
        check_eq(set.len(), 2)
    });
    case("sample-3", || {
        let versions = [ver(1, 0, 0, None), ver(2, 0, 0, Some("rc1")), ver(1, 4, 0, None)];
        check_eq(latest_stable(&versions), Some(&ver(1, 4, 0, None)))
    });
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
        parses_to("0010.02.3-007", ver(10, 2, 3, Some("007")))
    });
    case("hidden-5", || {
        // Sorting, max and min.
        let mut got = vec![
            ver(1, 10, 0, None),
            ver(1, 0, 0, None),
            ver(2, 0, 0, Some("alpha")),
            ver(1, 0, 0, Some("rc.2")),
            ver(10, 0, 0, None),
            ver(1, 0, 0, Some("rc.10")),
            ver(1, 0, 0, Some("Beta")),
            ver(1, 0, 0, Some("alpha")),
            ver(1, 0, 0, None),
            ver(1, 0, 1, Some("a")),
        ];
        got.sort();
        let expected = vec![
            ver(1, 0, 0, Some("Beta")),
            ver(1, 0, 0, Some("alpha")),
            ver(1, 0, 0, Some("rc.10")),
            ver(1, 0, 0, Some("rc.2")),
            ver(1, 0, 0, None),
            ver(1, 0, 0, None),
            ver(1, 0, 1, Some("a")),
            ver(1, 10, 0, None),
            ver(2, 0, 0, Some("alpha")),
            ver(10, 0, 0, None),
        ];
        check_eq(&got, &expected)?;
        let list = [ver(1, 9, 9, None), ver(2, 0, 0, Some("alpha")), ver(1, 10, 0, None)];
        check_eq(list.iter().max(), Some(&ver(2, 0, 0, Some("alpha"))))?;
        let list = [ver(3, 0, 0, None), ver(3, 0, 0, Some("rc.1")), ver(3, 0, 0, Some("beta"))];
        check_eq(list.iter().min(), Some(&ver(3, 0, 0, Some("beta"))))?;
        check_eq(list.into_iter().max(), Some(ver(3, 0, 0, None)))
    });
    case("hidden-6", || {
        // cmp, partial_cmp, the operators and == all agree.
        let list = [
            ver(1, 2, 0, None),
            ver(1, 2, 0, Some("rc1")),
            ver(1, 2, 0, Some("RC1")),
            ver(0, 10, 0, None),
            ver(1, 2, 0, None),
        ];
        for a in &list {
            for b in &list {
                let ord = a.cmp(b);
                let ok = a.partial_cmp(b) == Some(ord)
                    && (a == b) == (ord == Ordering::Equal)
                    && (a < b) == (ord == Ordering::Less);
                if !ok {
                    return Err(format!("inconsistent comparison between {a:?} and {b:?}"));
                }
            }
        }
        check_eq(list[0].cmp(&list[4]), Ordering::Equal)?;
        check_eq(list[0].partial_cmp(&list[1]), Some(Ordering::Greater))?;
        check(list[3] > ver(0, 9, 1, Some("x")), "0.10.0 > 0.9.1-x")?;
        ascending(&[
            ver(1, 2, 0, Some("rc")),
            ver(1, 2, 0, Some("rc1")),
            ver(1, 2, 0, Some("rc1.1")),
            ver(1, 2, 0, None),
        ])
    });
    case("hidden-7", || {
        // BTreeSet and HashSet see the same distinct versions.
        let list = [
            ver(1, 2, 0, Some("rc1")),
            v("01.02.00"),
            ver(1, 2, 0, Some("RC1")),
            ver(1, 2, 0, None),
            ver(2, 0, 0, None),
        ];
        let tree: BTreeSet<Version> = list.iter().cloned().collect();
        let hash: HashSet<Version> = list.into_iter().collect();
        check_eq(tree.len(), 4)?;
        check_eq(hash.len(), 4)?;
        let expected = vec![
            ver(1, 2, 0, Some("RC1")),
            ver(1, 2, 0, Some("rc1")),
            ver(1, 2, 0, None),
            ver(2, 0, 0, None),
        ];
        check_eq(tree.into_iter().collect::<Vec<_>>(), expected)
    });
    case("hidden-8", || {
        // latest_stable
        check_eq(latest_stable(&[]), None)?;
        check_eq(latest_stable(&[ver(1, 0, 0, Some("rc1")), ver(2, 0, 0, Some("alpha"))]), None)?;
        let versions = [
            ver(1, 9, 0, None),
            ver(2, 0, 0, Some("rc.1")),
            ver(1, 10, 0, None),
            ver(0, 5, 0, None),
            ver(1, 10, 0, None),
        ];
        check_eq(latest_stable(&versions), Some(&ver(1, 10, 0, None)))?;
        let versions = [
            ver(3, 0, 0, Some("beta")),
            ver(3, 0, 0, None),
            ver(3, 0, 0, Some("rc")),
            ver(2, 9, 0, None),
        ];
        check_eq(latest_stable(&versions), Some(&ver(3, 0, 0, None)))?;
        check_eq(latest_stable(&[ver(0, 0, 0, None)]), Some(&ver(0, 0, 0, None)))
    });
}
