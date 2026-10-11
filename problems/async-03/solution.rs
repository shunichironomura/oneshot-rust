/// An asynchronous key-value store. It is `Sync`, so that a future that holds a `&S` across an
/// `.await` (like those of `total` and `Fallback::get`) can be `Send`.
pub trait Store: Sync {
    /// The value stored for `key`, if any. Unlike `async fn`, this form can require the future to
    /// be `Send`. Implementations may still be written as `async fn`.
    fn get(&self, key: &str) -> impl Future<Output = Option<u32>> + Send;
}

/// A store that keeps its values in memory.
pub struct MemStore {
    pairs: Vec<(String, u32)>,
}

impl MemStore {
    pub fn new(pairs: &[(&str, u32)]) -> Self {
        MemStore { pairs: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect() }
    }
}

impl Store for MemStore {
    async fn get(&self, key: &str) -> Option<u32> {
        self.pairs.iter().find(|(k, _)| k == key).map(|&(_, value)| value)
    }
}

/// Asks `first`, and asks `second` only when `first` does not have the key.
pub struct Fallback<A, B> {
    pub first: A,
    pub second: B,
}

impl<A: Store, B: Store> Store for Fallback<A, B> {
    async fn get(&self, key: &str) -> Option<u32> {
        match self.first.get(key).await {
            Some(value) => Some(value),
            None => self.second.get(key).await,
        }
    }
}

/// The sum of the values of `keys`, looked up one at a time. Missing keys count as 0.
pub async fn total<S: Store>(s: &S, keys: &[&str]) -> u32 {
    let mut sum = 0;
    for key in keys {
        sum += s.get(key).await.unwrap_or(0);
    }
    sum
}
