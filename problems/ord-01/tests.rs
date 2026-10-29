use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};

use crate::judge::{case, check, check_eq};
use crate::user::{Version, latest_stable};

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

pub fn run() {
    case("example-1", || {
        ascending(&[ver(1, 9, 0, None), ver(1, 10, 0, None)])?;
        ascending(&[
            ver(1, 0, 0, Some("alpha")),
            ver(1, 0, 0, Some("alpha.1")),
            ver(1, 0, 0, Some("beta")),
            ver(1, 0, 0, None),
            ver(1, 0, 1, None),
        ])
    });
    case("example-2", || {
        let set: BTreeSet<Version> = ["1.2", "1.2.0", "1.2.0-rc1"].into_iter().map(v).collect();
        check_eq(set.len(), 2)
    });
    case("example-3", || {
        let versions = [ver(1, 0, 0, None), ver(2, 0, 0, Some("rc1")), ver(1, 4, 0, None)];
        check_eq(latest_stable(&versions), Some(&ver(1, 4, 0, None)))
    });
    case("case-1", || {
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
    case("case-2", || {
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
    case("case-3", || {
        // BTreeSet and HashSet see the same distinct versions.
        let list = [
            ver(1, 2, 0, Some("rc1")),
            v("1.2"),
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
    case("case-4", || {
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
