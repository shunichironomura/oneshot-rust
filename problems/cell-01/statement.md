This code compiles, and when you run it natively it seems to work. But it contains
**undefined behavior (UB)**, and Miri detects it.

`Lookup` is a table from names to numbers that also keeps statistics about how it is used.

```rust
impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup;
    pub fn insert(&mut self, key: &str, value: u32);
    pub fn get(&self, key: &str) -> Option<u32>;
    pub fn hits(&self) -> u32;
    pub fn misses(&self) -> u32;
    pub fn total(&self) -> u32;
    pub fn computations(&self) -> u32;
}
```

- `new` builds the table from `pairs`. The keys in `pairs` are distinct. All counts start at 0.
- `insert` adds `key` with `value`, or replaces the value if `key` is already in the table.
- `get` returns the value for `key` and counts a **hit**. If there is no such key, it returns `None`
  and counts a **miss**. `hits` and `misses` return those counts. Only `get` changes them.
- `total` returns the sum of all values in the table (`0` for an empty table). The sum is computed
  the first time `total` is called and then cached: later calls return the cached sum without
  computing it again. `insert` discards the cached sum, so the next `total` computes it again.
  `new` and `insert` never compute the sum themselves.
- `computations` returns how many times the sum has been computed so far.
- You may assume that every sum fits in a `u32`.

Do not change the public signatures: the tests call `get` and `total` through several shared
references (`&Lookup`) to one `Lookup` at the same time, and through an `Rc<Lookup>`. You may change
the private fields of `Lookup`.

Remove the UB so that it works correctly. (You may keep using `unsafe` if you like.)

### Examples

```rust
let mut lookup = Lookup::new(&[("a", 1), ("b", 2)]);
lookup.get("a"); // Some(1), a hit
lookup.get("z"); // None, a miss
lookup.total(); // 3 (computed)
lookup.total(); // 3 (cached)
// hits() == 1, misses() == 1, computations() == 1

lookup.insert("c", 10);
lookup.total(); // 13 (computed again)
// computations() == 2
```
