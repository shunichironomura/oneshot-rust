//! Judge library linked into every submission.
//!
//! Emits protocol lines of the form `\x01J:<nonce>:<body>` on stdout. The nonce
//! is substituted per submission so that player code cannot forge results.

use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::panic::{self, AssertUnwindSafe};

const NONCE: &str = "__NONCE__";

thread_local! {
    static LAST_PANIC: RefCell<Option<String>> = const { RefCell::new(None) };
    static IN_CASE: Cell<bool> = const { Cell::new(false) };
}

fn emit(body: &str) {
    println!("\x01J:{NONCE}:{body}");
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', "\\n").replace('\r', "\\r")
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    match payload.downcast_ref::<&str>() {
        Some(s) => s.to_string(),
        None => payload
            .downcast_ref::<String>()
            .cloned()
            .unwrap_or_else(|| "Box<dyn Any>".into()),
    }
}

pub fn start() {
    panic::set_hook(Box::new(|info| {
        let msg = panic_message(info.payload());
        let loc = info
            .location()
            .map(|l| format!(" at {}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_default();
        let text = format!("{msg}{loc}");
        if IN_CASE.with(Cell::get) {
            LAST_PANIC.with(|p| *p.borrow_mut() = Some(text));
        } else {
            // Panics outside of a test case are runtime errors; keep the message visible.
            eprintln!("panicked: {text}");
        }
    }));
    emit("START");
}

pub fn finish() {
    emit("END");
}

/// Runs one test case. Panics inside `f` are caught and reported as failures.
pub fn case(id: &str, f: impl FnOnce() -> Result<(), String>) {
    IN_CASE.with(|c| c.set(true));
    let result = panic::catch_unwind(AssertUnwindSafe(f));
    IN_CASE.with(|c| c.set(false));
    match result {
        Ok(Ok(())) => emit(&format!("CASE:{id}:PASS")),
        Ok(Err(m)) => emit(&format!("CASE:{id}:FAIL:{}", escape(&m))),
        Err(payload) => {
            let m = LAST_PANIC
                .with(|p| p.borrow_mut().take())
                .unwrap_or_else(|| panic_message(&*payload));
            emit(&format!("CASE:{id}:FAIL:panicked: {}", escape(&m)));
        }
    }
}

pub fn check_eq<T, U>(got: T, expected: U) -> Result<(), String>
where
    T: PartialEq<U> + Debug,
    U: Debug,
{
    if got == expected {
        Ok(())
    } else {
        Err(format!("expected {expected:?}, got {got:?}"))
    }
}

pub fn check(cond: bool, message: &str) -> Result<(), String> {
    if cond { Ok(()) } else { Err(message.to_string()) }
}
