`Registry<K, V>` wraps a `HashMap<K, V>` (it is already in the editor). The bodies of its methods are
already written, but four of the signatures accept only a `&K` as the key, so the test harness (the
`tests.rs` tab) does not compile: it looks up `String` keys with a `&str` and `Vec<u8>` keys with a
`&[u8]`, as a `HashMap` allows. Change **only the signatures** of `get`, `contains`, `remove` and
`get_or` so that the harness compiles and passes. Do not change the struct, the bodies, `new` or
`insert`. You may add `use` declarations.

```rust
impl<K: Hash + Eq, V> Registry<K, V> {
    pub fn new() -> Self;                              // unchanged
    pub fn insert(&mut self, k: K, v: V);              // unchanged
    pub fn get(&self, key: &K) -> Option<&V>;          // to fix
    pub fn contains(&self, key: &K) -> bool;           // to fix
    pub fn remove(&mut self, key: &K) -> Option<V>;    // to fix
    pub fn get_or(&self, key: &K, default: &V) -> &V;  // to fix
}
```

- `insert(k, v)` stores `v` under `k`. If `k` is already there, its value is replaced.
- `get(key)` returns the value stored under `key`, or `None` if there is none.
- `contains(key)` tells whether `key` is in the registry.
- `remove(key)` removes `key` and returns its value, or `None` if it was not there.
- `get_or(key, default)` returns the value stored under `key`, or `default` itself if `key` is
  missing.
- The harness uses a `Registry<String, u32>`, a `Registry<Vec<u8>, &str>` and a
  `Registry<u32, char>`. As with `HashMap`, a key can be looked up through a borrowed form of it:
  `&str` or `&String` for a `String` key, `&[u8]` or `&Vec<u8>` for a `Vec<u8>` key, and `&u32` for
  a `u32` key.
- The key is only used for the look-up. The results of `get` and `get_or` never borrow from it, so
  the key may be dropped before the result is used. The result of `get_or` borrows from the registry
  or from `default`.

### Examples

```rust
let mut r: Registry<String, u32> = Registry::new();
r.insert(String::from("apple"), 1);
r.insert(String::from("pear"), 2);
r.get("apple")                // Some(&1)
r.get(&String::from("pear"))  // Some(&2)
r.contains("pear")            // true
r.remove("apple")             // Some(1), and then r.get("apple") is None
let zero = 0;
r.get_or("zzz", &zero)        // &0
r.get_or("pear", &zero)       // &2

let mut bytes: Registry<Vec<u8>, &str> = Registry::new();
bytes.insert(vec![1, 2], "one-two");
bytes.get(&[1u8, 2][..])      // Some(&"one-two")

let mut chars: Registry<u32, char> = Registry::new();
chars.insert(7, 'x');
chars.get(&7)                 // Some(&'x')
```
