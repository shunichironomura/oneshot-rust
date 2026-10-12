use crate::judge::{case, check_eq};
use crate::user::History;

const NO_ACTIONS: [&str; 0] = [];

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
    case("hidden-1", || {
        // A capacity of 0 keeps nothing.
        let mut h = History::new(0);
        h.record("a");
        h.record("b");
        check_eq(h.len(), 0)?;
        check_eq(h.oldest(), None)?;
        check_eq(h.recent(3), NO_ACTIONS)?;
        check_eq(h.undo(), None)
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
        check_eq(h.undo(), None)
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
        // The same text recorded several times is several actions.
        let mut h = history(4, &["x", "x", "y", "x"]);
        check_eq(h.len(), 4)?;
        check_eq(h.recent(5), ["x", "y", "x", "x"])?;
        check_eq(h.undo().as_deref(), Some("x"))?;
        check_eq(h.len(), 3)?;
        check_eq(h.recent(5), ["y", "x", "x"])?;
        check_eq(h.oldest(), Some("x"))
    });
}
