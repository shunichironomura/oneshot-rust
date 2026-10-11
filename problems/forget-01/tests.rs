use std::cell::RefCell;
use std::rc::Rc;

use crate::judge::{case, check_eq};
use crate::user::{Log, TempFile};

fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// Returns the lines logged since the last call, and clears the log.
fn drain(log: &Log) -> Vec<String> {
    std::mem::take(&mut *log.borrow_mut())
}

fn nothing() -> Vec<String> {
    Vec::new()
}

/// Creates a file named `name` and writes `data` to it.
fn file(log: &Log, name: &str, data: &[u8]) -> TempFile {
    let mut f = TempFile::create(name, log);
    f.write(data);
    f
}

/// What `persist` should return.
fn kept(name: &str, data: &[u8]) -> (String, Vec<u8>) {
    (name.to_string(), data.to_vec())
}

/// Exactly `files` files that write to `log` are still alive (besides the test's own handle).
fn alive(log: &Log, files: usize) -> Result<(), String> {
    let handles = Rc::strong_count(log) - 1;
    if handles == files {
        Ok(())
    } else {
        Err(format!("{files} file(s) should keep the log alive, but {handles} handle(s) do"))
    }
}

pub fn run() {
    case("sample-1", || {
        let log = new_log();
        let mut a = TempFile::create("a", &log);
        a.write(b"hi");
        check_eq(a.persist(), kept("a", b"hi"))?;
        alive(&log, 0)?;
        let b = TempFile::create("b", &log);
        drop(b);
        check_eq(drain(&log), ["create a", "create b", "delete b (0 bytes)"])?;
        alive(&log, 0)
    });
    case("sample-2", || {
        let log = new_log();
        let mut f = TempFile::create("draft", &log);
        f.write(b"abc");
        let f = f.rename("final");
        check_eq(drain(&log), ["create draft", "rename draft -> final"])?;
        alive(&log, 1)?;
        drop(f);
        check_eq(drain(&log), ["delete final (3 bytes)"])?;
        alive(&log, 0)
    });
    case("hidden-1", || {
        // An empty name and empty contents are nothing special.
        let log = new_log();
        let f = TempFile::create("", &log);
        check_eq(f.persist(), kept("", b""))?;
        check_eq(drain(&log), ["create "])?;
        alive(&log, 0)?;
        let g = TempFile::create("", &log);
        drop(g);
        check_eq(drain(&log), ["create ", "delete  (0 bytes)"])?;
        alive(&log, 0)
    });
    case("hidden-2", || {
        // Writes add up, and lengths count bytes.
        let log = new_log();
        let mut f = TempFile::create("notes.txt", &log);
        f.write(b"ab");
        f.write(b"");
        f.write("é€".as_bytes());
        f.write(&[0, 255]);
        let mut g = file(&log, "ログ", "é€".as_bytes());
        g.write(b"!");
        drop(f);
        check_eq(drain(&log), ["create notes.txt", "create ログ", "delete notes.txt (9 bytes)"])?;
        alive(&log, 1)?;
        check_eq(g.persist(), kept("ログ", "é€!".as_bytes()))?;
        check_eq(drain(&log), nothing())?;
        alive(&log, 0)
    });
    case("hidden-3", || {
        // Renaming several times, writing in between, then persisting.
        let log = new_log();
        let f = file(&log, "a", b"x");
        let mut f = f.rename("b");
        f.write(b"y");
        let f = f.rename("c").rename("c").rename("");
        alive(&log, 1)?;
        check_eq(f.persist(), kept("", b"xy"))?;
        check_eq(
            drain(&log),
            ["create a", "rename a -> b", "rename b -> c", "rename c -> c", "rename c -> "],
        )?;
        alive(&log, 0)
    });
    case("hidden-4", || {
        // Several files share a log: persisting or renaming one leaves the others alone.
        let log = new_log();
        {
            let _a = file(&log, "a", b"1");
            let b = file(&log, "b", b"22");
            let c = file(&log, "c", b"333");
            let _b = b.rename("b2");
            alive(&log, 3)?;
            check_eq(c.persist(), kept("c", b"333"))?;
            alive(&log, 2)?;
        }
        check_eq(
            drain(&log),
            [
                "create a",
                "create b",
                "create c",
                "rename b -> b2",
                "delete b2 (2 bytes)",
                "delete a (1 bytes)",
            ],
        )?;
        alive(&log, 0)
    });
    case("hidden-5", || {
        // Files kept in collections, on two logs.
        let (log1, log2) = (new_log(), new_log());
        let mut files = vec![
            file(&log1, "p", b"pp"),
            file(&log2, "q", b"q"),
            file(&log1, "r", b""),
            file(&log2, "s", b"ssss"),
        ];
        check_eq(files.pop().map(TempFile::persist), Some(kept("s", b"ssss")))?;
        let q = files.remove(1).rename("q2");
        alive(&log2, 1)?;
        let mut saved = Vec::new();
        for (i, f) in files.into_iter().enumerate() {
            if i == 0 {
                saved.push(f.persist());
            }
        }
        check_eq(saved, [kept("p", b"pp")])?;
        check_eq(drain(&log1), ["create p", "create r", "delete r (0 bytes)"])?;
        alive(&log1, 0)?;
        let slot = Some(q);
        drop(slot);
        check_eq(drain(&log2), ["create q", "create s", "rename q -> q2", "delete q2 (1 bytes)"])?;
        alive(&log2, 0)
    });
    case("hidden-6", || {
        // A variable whose file was persisted or renamed can be assigned again.
        let log = new_log();
        let mut f = file(&log, "one", b"1");
        check_eq(f.persist(), kept("one", b"1"))?;
        f = file(&log, "two", b"22");
        f = f.rename("three");
        f.write(b"3");
        alive(&log, 1)?;
        f = file(&log, "four", b"");
        check_eq(
            drain(&log),
            ["create one", "create two", "rename two -> three", "create four", "delete three (3 bytes)"],
        )?;
        check_eq(f.persist(), kept("four", b""))?;
        check_eq(drain(&log), nothing())?;
        alive(&log, 0)
    });
}
