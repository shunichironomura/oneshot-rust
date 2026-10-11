The code in the editor does not compile. **Without looking at the error message**, predict where
the compiler complains and fix it.

A small shop is split into modules: `shop` has the items, `shop::cart` the shopping cart and
`shop::pricing` the price calculations. Your code is the module `crate::user`. The test harness is
another module of the same crate, outside `crate::user`, and it uses exactly these paths:

```rust
crate::user::shop::TAX_RATE // a u32 constant, 10
crate::user::shop::Item::new(name: &str, price: u32) -> Item
crate::user::shop::Item::name(&self) -> &str
crate::user::shop::Item::price(&self) -> u32
crate::user::shop::cart::Cart::new() -> Cart
crate::user::shop::cart::Cart::add(&mut self, item: Item)
crate::user::shop::cart::Cart::total(&self) -> u32
crate::user::shop::cart::Cart::len(&self) -> usize
crate::user::sample_cart() -> Cart
crate::user::checkout(cart: &Cart, percent: u32) -> u32
```

- Prices are in cents. Every division is an integer division, which rounds down.
- `Item::new` creates an item, and `name` and `price` return the values it was created with.
- `Cart::new` creates an empty cart and `add` puts an item in it. `total` is the sum of the prices
  of all items in the cart (`0` for an empty cart), and `len` is the number of items.
- `sample_cart` returns a new cart holding a pen for `120` and a book for `1500`.
- `checkout(cart, percent)` first takes the discount `t * percent / 100` off the cart's total `t`,
  then adds the tax `d * TAX_RATE / 100` to the discounted price `d`. `percent` is between `0` and
  `100`.
- Keep the fields of `Item` and `Cart` private, and do not change what the code computes. You may
  change visibility, paths and `use` declarations, and rewrite how `sample_cart` and `checkout` do
  their work.

### Example

`sample_cart().total()` is `1620`, and `checkout(&sample_cart(), 10)` is `1603`: the discount is
`162`, which leaves `1458`, and the tax on that is `145`.
