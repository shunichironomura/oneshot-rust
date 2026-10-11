use std::cell::{Cell, OnceCell};
use std::collections::HashMap;

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: Cell<u32>,
    misses: Cell<u32>,
    keys: OnceCell<Vec<String>>,
    sorts: Cell<u32>,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: Cell::new(0),
            misses: Cell::new(0),
            keys: OnceCell::new(),
            sorts: Cell::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        // `&mut self` guarantees that no slice returned by `keys` is still in use.
        self.keys = OnceCell::new();
    }

    pub fn get(&self, key: &str) -> Option<u32> {
        let value = self.table.get(key).copied();
        let counter = if value.is_some() { &self.hits } else { &self.misses };
        counter.set(counter.get() + 1);
        value
    }

    pub fn hits(&self) -> u32 {
        self.hits.get()
    }

    pub fn misses(&self) -> u32 {
        self.misses.get()
    }

    pub fn keys(&self) -> &[String] {
        // Through `&self`, a `OnceCell` is only ever filled, never changed, so it can hand out a
        // plain reference to its contents.
        self.keys.get_or_init(|| {
            self.sorts.set(self.sorts.get() + 1);
            let mut keys: Vec<String> = self.table.keys().cloned().collect();
            keys.sort_unstable();
            keys
        })
    }

    pub fn sorts(&self) -> u32 {
        self.sorts.get()
    }
}
