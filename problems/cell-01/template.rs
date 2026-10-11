use std::collections::HashMap;

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: u32,
    misses: u32,
    keys: Option<Vec<String>>,
    sorts: u32,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: 0,
            misses: 0,
            keys: None,
            sorts: 0,
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        self.keys = None;
    }

    pub fn get(&self, key: &str) -> Option<u32> {
        let value = self.table.get(key).copied();
        let counter = if value.is_some() {
            &raw const self.hits
        } else {
            &raw const self.misses
        };
        // SAFETY: the counters are plain numbers, and nobody holds a `&mut` to them.
        unsafe { *(counter as *mut u32) += 1 };
        value
    }

    pub fn hits(&self) -> u32 {
        self.hits
    }

    pub fn misses(&self) -> u32 {
        self.misses
    }

    pub fn keys(&self) -> &[String] {
        if self.keys.is_none() {
            let mut keys: Vec<String> = self.table.keys().cloned().collect();
            keys.sort();
            // SAFETY: same as in `get`.
            unsafe {
                *(&raw const self.keys as *mut Option<Vec<String>>) = Some(keys);
                *(&raw const self.sorts as *mut u32) += 1;
            }
        }
        self.keys.as_deref().unwrap()
    }

    pub fn sorts(&self) -> u32 {
        self.sorts
    }
}
