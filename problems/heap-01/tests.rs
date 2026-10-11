use crate::judge::{case, check_eq};
use crate::user::{Triage, k_smallest};

/// Calls `next` until it returns `None` and collects the names.
fn drain(t: &mut Triage) -> Vec<String> {
    std::iter::from_fn(|| t.next()).collect()
}

pub fn run() {
    case("sample-1", || {
        let mut t = Triage::new();
        t.arrive("ann", 2);
        t.arrive("bob", 5);
        t.arrive("cid", 5);
        t.arrive("dan", 1);
        check_eq(t.peek(), Some("bob"))?;
        check_eq(t.len(), 4)?;
        check_eq(drain(&mut t), vec!["bob", "cid", "ann", "dan"])?;
        check_eq(t.next(), None)
    });
    case("sample-2", || check_eq(k_smallest(&[5, 1, 4, 1, 3], 3), vec![1, 1, 3]));
    case("hidden-1", || {
        // Nobody waiting, then a single patient.
        let mut t = Triage::new();
        check_eq(t.len(), 0)?;
        check_eq(t.peek(), None)?;
        check_eq(t.next(), None)?;
        t.arrive("solo", 4);
        check_eq(t.peek(), Some("solo"))?;
        check_eq(t.len(), 1)?;
        check_eq(t.next(), Some("solo".to_string()))?;
        check_eq(t.len(), 0)?;
        check_eq(t.peek(), None)?;
        check_eq(t.next(), None)
    });
    case("hidden-2", || {
        // Same severity: arrival order, whatever the names are.
        let mut t = Triage::new();
        for name in ["kim", "al", "zoe", "bo", "max"] {
            t.arrive(name, 3);
        }
        check_eq(drain(&mut t), vec!["kim", "al", "zoe", "bo", "max"])
    });
    case("hidden-3", || {
        // Arrivals and departures interleaved; peek does not remove anyone.
        let mut t = Triage::new();
        t.arrive("uma", 7);
        t.arrive("ivy", 7);
        t.arrive("lou", 7);
        check_eq(t.next(), Some("uma".to_string()))?;
        check_eq(t.next(), Some("ivy".to_string()))?;
        t.arrive("abe", 7);
        check_eq(t.peek(), Some("lou"))?;
        check_eq(t.peek(), Some("lou"))?;
        check_eq(t.len(), 2)?;
        t.arrive("fay", 9);
        check_eq(t.peek(), Some("fay"))?;
        check_eq(t.len(), 3)?;
        check_eq(drain(&mut t), vec!["fay", "lou", "abe"])?;
        check_eq(t.len(), 0)
    });
    case("hidden-4", || {
        // Extreme severities, and two different patients with the same name.
        let mut t = Triage::new();
        t.arrive("max", 0);
        t.arrive("sam", 255);
        t.arrive("ada", 0);
        t.arrive("sam", 128);
        t.arrive("bea", 255);
        t.arrive("cy", 127);
        check_eq(t.len(), 6)?;
        check_eq(drain(&mut t), vec!["sam", "bea", "sam", "cy", "max", "ada"])
    });
    case("hidden-5", || {
        check_eq(k_smallest(&[4, 2, 9], 0), vec![])?;
        check_eq(k_smallest(&[], 3), vec![])?;
        check_eq(k_smallest(&[], 0), vec![])
    });
    case("hidden-6", || {
        // k is v.len() or more: everything, sorted.
        let v = [3, -2, 7, -2];
        check_eq(k_smallest(&v, 4), vec![-2, -2, 3, 7])?;
        check_eq(k_smallest(&v, 10), vec![-2, -2, 3, 7])?;
        check_eq(k_smallest(&v, usize::MAX), vec![-2, -2, 3, 7])
    });
    case("hidden-7", || {
        let v = [i32::MAX, -5, 10, i32::MIN, 0, -5, 7, 6, 8, -1];
        check_eq(k_smallest(&v, 4), vec![i32::MIN, -5, -5, -1])?;
        check_eq(k_smallest(&v, 1), vec![i32::MIN])?;
        check_eq(k_smallest(&[i32::MAX, i32::MAX, 8], 2), vec![8, i32::MAX])
    });
}
