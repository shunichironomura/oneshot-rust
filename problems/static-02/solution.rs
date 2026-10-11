use std::any::TypeId;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The counter of every type that has been registered at least once.
static COUNTS: Mutex<BTreeMap<TypeId, usize>> = Mutex::new(BTreeMap::new());

pub fn register<T: 'static>() -> usize {
    let mut counts = COUNTS.lock().unwrap();
    let n = counts.entry(TypeId::of::<T>()).or_insert(0);
    *n += 1;
    *n
}

pub fn count<T: 'static>() -> usize {
    let counts = COUNTS.lock().unwrap();
    counts.get(&TypeId::of::<T>()).copied().unwrap_or(0)
}

pub fn total() -> usize {
    COUNTS.lock().unwrap().values().sum()
}

pub fn distinct() -> usize {
    COUNTS.lock().unwrap().len()
}
