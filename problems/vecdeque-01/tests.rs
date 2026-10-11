use crate::judge::{case, check_eq};
use crate::user::{History, josephus};

const NO_ACTIONS: [&str; 0] = [];
const NOBODY: [usize; 0] = [];

fn history(capacity: usize, actions: &[&str]) -> History {
    let mut h = History::new(capacity);
    for action in actions {
        h.record(action);
    }
    h
}

pub fn run() {
    case("sample-1", || {
        let mut h = history(3, &["type", "bold", "paste", "delete"]);
        check_eq(h.len(), 3)?;
        check_eq(h.oldest(), Some("bold"))?;
        check_eq(h.recent(2), ["delete", "paste"])?;
        check_eq(h.recent(10), ["delete", "paste", "bold"])?;
        check_eq(h.undo().as_deref(), Some("delete"))?;
        check_eq(h.recent(10), ["paste", "bold"])
    });
    case("sample-2", || check_eq(josephus(7, 3), [3, 6, 2, 7, 5, 1, 4]));
    case("hidden-1", || {
        // A capacity of 0 keeps nothing.
        let mut h = History::new(0);
        h.record("a");
        check_eq(h.len(), 0)?;
        h.record("b");
        check_eq(h.len(), 0)?;
        check_eq(h.oldest(), None)?;
        check_eq(h.recent(3), NO_ACTIONS)?;
        check_eq(h.undo(), None)?;
        check_eq(h.len(), 0)
    });
    case("hidden-2", || {
        // An empty history, then a single action.
        let mut h = History::new(3);
        check_eq(h.len(), 0)?;
        check_eq(h.undo(), None)?;
        check_eq(h.oldest(), None)?;
        check_eq(h.recent(0), NO_ACTIONS)?;
        check_eq(h.recent(5), NO_ACTIONS)?;
        h.record("solo");
        check_eq(h.len(), 1)?;
        check_eq(h.recent(0), NO_ACTIONS)?;
        check_eq(h.recent(1), ["solo"])?;
        check_eq(h.oldest(), Some("solo"))?;
        check_eq(h.undo().as_deref(), Some("solo"))?;
        check_eq(h.undo(), None)?;
        check_eq(h.oldest(), None)?;
        check_eq(h.len(), 0)
    });
    case("hidden-3", || {
        // A capacity of 1 keeps only the newest action.
        let mut h = history(1, &["a", "b", "c"]);
        check_eq(h.len(), 1)?;
        check_eq(h.oldest(), Some("c"))?;
        check_eq(h.recent(5), ["c"])?;
        check_eq(h.undo().as_deref(), Some("c"))?;
        check_eq(h.undo(), None)?;
        h.record("d");
        check_eq(h.recent(usize::MAX), ["d"])
    });
    case("hidden-4", || {
        // Newest first for every n, including n larger than the history.
        let h = history(10, &["1", "2", "3", "4", "5", "6"]);
        check_eq(h.len(), 6)?;
        check_eq(h.oldest(), Some("1"))?;
        check_eq(h.recent(1), ["6"])?;
        check_eq(h.recent(3), ["6", "5", "4"])?;
        check_eq(h.recent(6), ["6", "5", "4", "3", "2", "1"])?;
        check_eq(h.recent(7), ["6", "5", "4", "3", "2", "1"])?;
        check_eq(h.recent(usize::MAX), ["6", "5", "4", "3", "2", "1"])
    });
    case("hidden-5", || {
        // Dropped actions never come back, and undo frees a slot.
        let mut h = history(3, &["a", "b", "c", "d", "e"]);
        check_eq(h.recent(10), ["e", "d", "c"])?;
        check_eq(h.undo().as_deref(), Some("e"))?;
        h.record("f");
        check_eq(h.recent(10), ["f", "d", "c"])?;
        check_eq(h.oldest(), Some("c"))?;
        h.record("g");
        check_eq(h.oldest(), Some("d"))?;
        check_eq(h.undo().as_deref(), Some("g"))?;
        check_eq(h.undo().as_deref(), Some("f"))?;
        check_eq(h.undo().as_deref(), Some("d"))?;
        check_eq(h.undo(), None)?;
        check_eq(h.len(), 0)?;
        check_eq(h.oldest(), None)
    });
    case("hidden-6", || {
        // The same text recorded twice is two actions.
        let mut h = history(3, &["x", "x", "y", "x"]);
        check_eq(h.recent(5), ["x", "y", "x"])?;
        check_eq(h.undo().as_deref(), Some("x"))?;
        check_eq(h.len(), 2)?;
        check_eq(h.recent(5), ["y", "x"])?;
        check_eq(h.oldest(), Some("x"))
    });
    case("hidden-7", || {
        check_eq(josephus(0, 3), NOBODY)?;
        check_eq(josephus(1, 1), [1])?;
        check_eq(josephus(5, 1), [1, 2, 3, 4, 5])?;
        check_eq(josephus(2, 2), [2, 1])?;
        check_eq(josephus(5, 5), [5, 1, 3, 4, 2])?;
        check_eq(josephus(10, 2), [2, 4, 6, 8, 10, 3, 7, 1, 9, 5])
    });
    case("hidden-8", || {
        // k larger than the circle, up to usize::MAX.
        check_eq(josephus(3, 10), [1, 3, 2])?;
        check_eq(josephus(1, 7), [1])?;
        check_eq(
            josephus(20, 7),
            [7, 14, 1, 9, 17, 5, 15, 4, 16, 8, 20, 13, 11, 10, 12, 19, 6, 18, 2, 3],
        )?;
        check_eq(josephus(8, 1_000_000_000), [8, 6, 3, 2, 1, 4, 7, 5])?;
        check_eq(josephus(6, usize::MAX), [3, 2, 6, 5, 1, 4])?;
        check_eq(josephus(1, usize::MAX), [1])
    });
}
