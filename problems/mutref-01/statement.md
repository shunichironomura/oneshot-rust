The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

`Labels` hands out the label `"item-<id>"` for an id. Building a label is assumed to be expensive, so
each label is built **only once**, kept in a cache, and returned as a reference into the cache from
then on. `built` reports how many labels have been built so far.

```rust
impl Labels {
    pub fn new() -> Labels
    pub fn get(&mut self, id: u32) -> &str
    pub fn built(&self) -> usize
}
```

Do not change these signatures.

### Example

Calling `get(3)` twice returns `"item-3"` both times, and `built()` is then `1`.
