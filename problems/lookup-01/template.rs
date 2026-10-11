use std::collections::HashMap;
use std::hash::Hash;

/// A map from keys to values that can be searched with borrowed forms of its keys.
pub struct Registry<K, V> {
    map: HashMap<K, V>,
}

impl<K: Hash + Eq, V> Registry<K, V> {
    pub fn new() -> Self {
        Registry { map: HashMap::new() }
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.map.insert(k, v);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }

    pub fn get_or(&self, key: &K, default: &V) -> &V {
        self.map.get(key).unwrap_or(default)
    }
}
