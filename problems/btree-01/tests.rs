use crate::judge::{case, check_eq};
use crate::user::Timeline;

const NONE: [(u32, &str); 0] = [];
const MAX: u32 = u32::MAX;

fn day() -> Timeline {
    let mut tl = Timeline::new();
    tl.add(5, "wake");
    tl.add(10, "work");
    tl.add(20, "sleep");
    tl
}

pub fn run() {
    case("sample-1", || {
        let mut tl = day();
        check_eq(tl.len(), 3)?;
        check_eq(tl.between(6, 20), [(10, "work"), (20, "sleep")])?;
        check_eq(tl.last_before(10), Some((5, "wake")))?;
        check_eq(tl.last_before(100), Some((20, "sleep")))?;
        check_eq(tl.add(10, "lunch"), Some("work".to_string()))?;
        check_eq(tl.len(), 3)
    });
    case("sample-2", || {
        let mut tl = day();
        let later = tl.split_from(10);
        check_eq(tl.between(0, 100), [(5, "wake")])?;
        check_eq(later.between(0, 100), [(10, "work"), (20, "sleep")])
    });
    case("hidden-1", || {
        // Both ends are included; `from > to` gives an empty result, not a panic.
        let mut tl = day();
        check_eq(tl.between(10, 10), [(10, "work")])?;
        check_eq(tl.between(9, 3), NONE)?;
        tl.add(MAX, "end");
        tl.add(0, "start");
        check_eq(tl.between(MAX, MAX), [(MAX, "end")])?;
        check_eq(
            tl.between(0, MAX),
            [(0, "start"), (5, "wake"), (10, "work"), (20, "sleep"), (MAX, "end")],
        )?;
        check_eq(tl.between(MAX, 0), NONE)
    });
    case("hidden-2", || {
        // last_before is strictly before t.
        let mut tl = day();
        check_eq(tl.last_before(0), None)?;
        check_eq(tl.last_before(5), None)?;
        check_eq(tl.last_before(20), Some((10, "work")))?;
        check_eq(tl.last_before(MAX), Some((20, "sleep")))?;
        tl.add(MAX, "end");
        tl.add(0, "start");
        check_eq(tl.last_before(0), None)?;
        check_eq(tl.last_before(MAX), Some((20, "sleep")))?;
        check_eq(tl.last_before(1), Some((0, "start")))?;
        check_eq(tl.last_before(21), Some((20, "sleep")))
    });
    case("hidden-3", || {
        // An empty timeline, then adding to it.
        let mut tl = Timeline::new();
        check_eq(tl.len(), 0)?;
        check_eq(tl.between(0, 100), NONE)?;
        check_eq(tl.last_before(5), None)?;
        check_eq(tl.split_from(3).len(), 0)?;
        check_eq(tl.add(7, "a"), None)?;
        check_eq(tl.add(3, "b"), None)?;
        check_eq(tl.add(7, "c"), Some("a".to_string()))?;
        check_eq(tl.add(7, "d"), Some("c".to_string()))?;
        check_eq(tl.len(), 2)?;
        check_eq(tl.between(0, 10), [(3, "b"), (7, "d")])
    });
    case("hidden-4", || {
        // split_from keeps times >= t in the new timeline and times < t in self.
        let mut tl = day();
        let mut all = tl.split_from(0);
        check_eq((tl.len(), all.len()), (0, 3))?;
        let none = all.split_from(21);
        check_eq((all.len(), none.len()), (3, 0))?;
        all.add(MAX, "end");
        all.add(0, "start");
        let mut late = all.split_from(20);
        check_eq(all.between(0, 100), [(0, "start"), (5, "wake"), (10, "work")])?;
        check_eq(late.between(0, MAX), [(20, "sleep"), (MAX, "end")])?;
        let end = late.split_from(MAX);
        check_eq(late.between(0, MAX), [(20, "sleep")])?;
        check_eq(end.between(0, MAX), [(MAX, "end")])
    });
}
