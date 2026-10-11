use std::borrow::Borrow;
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

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get(key)
    }

    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.contains_key(key)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove(key)
    }

    pub fn get_or<'a, Q>(&'a self, key: &Q, default: &'a V) -> &'a V
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get(key).unwrap_or(default)
    }
}
