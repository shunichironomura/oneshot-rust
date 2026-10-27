The code below does not compile. Fix it.

A `Cart` holds items. `checkout` recomputes two things from scratch:

- `total()`: the sum of `price * qty` over all items.
- `biggest()`: the name of the item with the largest subtotal `price * qty`. On a tie, the earlier
  item wins. It is `None` for an empty cart.

```rust
impl Cart {
    pub fn new(items: Vec<Item>) -> Self;
    pub fn checkout(&mut self);
    pub fn total(&self) -> u32;
    pub fn biggest(&self) -> Option<&str>;
}
```

Do not change the public signatures. Private helpers may be changed or removed.

### Example

For apple (120 × 3), melon (500 × 1) and tea (150 × 2), after `checkout()` the total is `1160` and
the biggest is `Some("melon")`.
