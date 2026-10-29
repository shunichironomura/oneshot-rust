use std::cell::RefCell;

use crate::judge::{case, check_eq};
use crate::user::{Tracer, trace};

pub fn run() {
    case("example-1", || {
        check_eq(trace(&["a", "b"]), vec!["begin a", "begin b", "run", "end a", "end b"])
    });
    case("case-1", || check_eq(trace(&[]), vec!["run"]));
    case("case-2", || check_eq(trace(&["solo"]), vec!["begin solo", "run", "end solo"]));
    case("case-3", || {
        // The tracers end in creation order, and a repeated name gets its own tracer.
        check_eq(
            trace(&["parse", "check", "lower", "check", "emit"]),
            vec![
                "begin parse",
                "begin check",
                "begin lower",
                "begin check",
                "begin emit",
                "run",
                "end parse",
                "end check",
                "end lower",
                "end check",
                "end emit",
            ],
        )
    });
    case("case-4", || {
        // `Tracer` is unchanged: `new` writes the "begin" line and dropping writes the "end" line.
        let sink = RefCell::new(Vec::new());
        let first = Tracer::new(&sink, "first");
        let second = Tracer::new(&sink, "second");
        check_eq(sink.borrow().clone(), vec!["begin first", "begin second"])?;
        drop(second);
        drop(first);
        check_eq(sink.into_inner(), vec!["begin first", "begin second", "end second", "end first"])
    });
}
