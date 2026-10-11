use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex, RwLock};

/// Maps each interned string to its id.
static IDS: LazyLock<Mutex<HashMap<String, u32>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
/// `NAMES[id]` is the string with that id.
static NAMES: RwLock<Vec<String>> = RwLock::new(Vec::new());
/// How many times `intern` has been called.
static CALLS: AtomicUsize = AtomicUsize::new(0);

pub fn intern(s: &str) -> u32 {
    CALLS.fetch_add(1, Ordering::Relaxed);
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
    CALLS.load(Ordering::Relaxed)
}
