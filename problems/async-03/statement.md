The trait `Store` describes an asynchronous key-value store, and the async function `total` adds up
values from any store. The code in the editor compiles on its own, but not together with the test
harness (the `tests.rs` tab): the harness creates the future of `total` on one thread and runs it to
completion on another, as a multi-threaded executor may do.

Change **only the trait `Store`** so that the harness compiles. Everything else (`MemStore`,
`Fallback`, their `impl Store` blocks and `total`) must stay exactly as it is. You may add `use`
declarations.

```rust
pub trait Store {
    async fn get(&self, key: &str) -> Option<u32>;
}

pub struct MemStore { /* private */ }

impl MemStore {
    pub fn new(pairs: &[(&str, u32)]) -> Self;
}

pub struct Fallback<A, B> {
    pub first: A,
    pub second: B,
}

impl Store for MemStore { /* async fn get */ }
impl<A: Store, B: Store> Store for Fallback<A, B> { /* async fn get */ }

pub async fn total<S: Store>(s: &S, keys: &[&str]) -> u32;
```

- `get(key)` on a `MemStore` resolves to the value paired with `key` in `pairs`, or to `None` if
  there is none. The keys in `pairs` are all different.
- `get(key)` on a `Fallback` asks `first`. Only when that resolves to `None` does it ask `second`,
  and then it resolves to the answer of `second`.
- `total(s, keys)` asks `s` for each key of `keys`, in order and one lookup at a time, and resolves
  to the sum of the values, counting missing keys as `0` (the sum is `0` when `keys` is empty). A
  key that appears several times is looked up each time. The sum always fits in a `u32`.
- The harness also implements `Store` for a store of its own, with an `async fn get` that logs the
  keys it is asked for and is not ready the first time it is polled. It uses this store both on its
  own and inside a `Fallback`.

### Examples

- For `MemStore::new(&[("a", 1), ("b", 2)])`, `total` with the keys `["a", "b", "zz"]` resolves to
  `3`.
- For a `Fallback` whose `first` is `MemStore::new(&[("a", 1)])` and whose `second` is
  `MemStore::new(&[("a", 100), ("b", 20)])`, `total` with the keys `["a", "b"]` resolves to `21`:
  `"a"` comes from `first` and `"b"` from `second`.
