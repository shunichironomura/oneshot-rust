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
    pub fn keys(&self) -> &[String];
    pub fn sorts(&self) -> u32;
}
```

- `new` builds the table from `pairs`. The keys in `pairs` are distinct. All counts start at 0.
- `insert` adds `key` with `value`, or replaces the value if `key` is already in the table.
- `get` returns the value for `key` and counts a **hit**. If there is no such key, it returns `None`
  and counts a **miss**. `hits` and `misses` return those counts. Only `get` changes them.
- `keys` returns all the keys of the table in ascending order, as `<` compares strings (byte by
  byte, so `"10" < "9"` and `"Z" < "a"`). For an empty table, it returns an empty slice.
- The list is sorted the first time `keys` is called and then cached: later calls return the
  cached list without sorting again. `insert` discards the cached list (even if the key was already
  in the table), so the next `keys` sorts again. `new` and `insert` never sort the keys themselves.
- `sorts` returns how many times the list has been sorted so far.

Do not change the public signatures: the tests call `get` and `keys` through several shared
references (`&Lookup`) to one `Lookup` at the same time, and through an `Rc<Lookup>`, and keep
using a slice returned by `keys` while they call other methods. You may change the private fields
of `Lookup`.

Remove the UB so that it works correctly. (You may keep using `unsafe` if you like.)

### Examples

```rust
let mut lookup = Lookup::new(&[("b", 2), ("a", 1)]);
lookup.get("a"); // Some(1), a hit
lookup.get("z"); // None, a miss
lookup.keys(); // ["a", "b"] (sorted)
lookup.keys(); // ["a", "b"] (cached)
// hits() == 1, misses() == 1, sorts() == 1

lookup.insert("c", 10);
lookup.keys(); // ["a", "b", "c"] (sorted again)
// sorts() == 2
```
