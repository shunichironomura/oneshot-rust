use std::collections::HashMap;

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: u32,
    misses: u32,
    total: Option<u32>,
    computations: u32,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: 0,
            misses: 0,
            total: None,
            computations: 0,
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        self.total = None;
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

    pub fn total(&self) -> u32 {
        if let Some(total) = self.total {
            return total;
        }
        let total = self.table.values().sum();
        // SAFETY: same as in `get`.
        unsafe {
            *(&raw const self.total as *mut Option<u32>) = Some(total);
            *(&raw const self.computations as *mut u32) += 1;
        }
        total
    }

    pub fn computations(&self) -> u32 {
        self.computations
    }
}
