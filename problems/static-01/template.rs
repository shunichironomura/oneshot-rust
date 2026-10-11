use std::cell::Cell;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

/// Maps each interned string to its id.
static IDS: Mutex<HashMap<String, u32>> = Mutex::new(HashMap::new());
/// `NAMES[id]` is the string with that id.
static NAMES: RwLock<Vec<String>> = RwLock::new(Vec::new());
/// How many times `intern` has been called.
static CALLS: Cell<usize> = Cell::new(0);

pub fn intern(s: &str) -> u32 {
    CALLS.set(CALLS.get() + 1);
    let mut ids = IDS.lock().unwrap();
    if let Some(&id) = ids.get(s) {
        return id;
    }
    // `IDS` stays locked until the new string is in both tables.
    let mut names = NAMES.write().unwrap();
    let id = names.len() as u32;
    names.push(s.to_string());
    ids.insert(s.to_string(), id);
    id
}

pub fn resolve(id: u32) -> Option<String> {
    NAMES.read().unwrap().get(id as usize).cloned()
}

pub fn interned_count() -> usize {
    NAMES.read().unwrap().len()
}

pub fn intern_calls() -> usize {
    CALLS.get()
}
