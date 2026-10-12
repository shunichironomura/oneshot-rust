Privacy in Rust is decided per module: an item without `pub` is visible in the module that defines
it and in all of that module's descendants, and nowhere else. `pub` on a struct only makes the type
itself visible; each field, method and associated function has its own visibility.

The template has three problems:

- `pricing` is a private module of `shop`, so `checkout`, which is outside `shop`, cannot reach it,
  even though its functions are `pub`: every module along a path must be visible too.
  `pub(super) mod pricing` opens it to the parent of `shop`, where `checkout` is, and no further.
  Plain `pub` works as well.
- `Item::price` is private, and the harness, which is outside `crate::user`, calls it. It needs
  `pub` (or `pub(crate)`). `pub(super)` is not enough, because it only reaches `crate::user`.
- `sample_cart` builds a `Cart` with a struct literal, but `items` is private to `shop::cart`. A
  struct literal names every field, so all of them must be visible where it is written. With the
  fields kept private, build the cart through its public API, `Cart::new` and `add`. The compiler
  checks field privacy in a later pass, so it reports this error only after the others are fixed.

One line looks suspicious but is fine: `Cart::total` reads `item.price`, a private field of `Item`.
The field is private to `shop`, and `shop::cart` is inside `shop`, so it can see it. Making `price`
a `pub` field would also compile, but it would expose the field to everyone, which the statement
forbids.

### Alternatives

`pricing` can stay private. Move `checkout` into `shop`, where `pricing` is visible, and re-export
it: `pub use` makes it available as `crate::user::checkout`, the path the harness uses.

```rust
pub mod shop {
    // TAX_RATE, Item and cart as in the solution.

    mod pricing {
        pub fn discounted(total: u32, percent: u32) -> u32 {
            total - total * percent / 100
        }

        pub fn with_tax(total: u32) -> u32 {
            total + total * super::TAX_RATE / 100
        }
    }

    pub fn checkout(cart: &cart::Cart, percent: u32) -> u32 {
        pricing::with_tax(pricing::discounted(cart.total(), percent))
    }
}

pub use shop::checkout;

// sample_cart as in the solution.
```

`use` declarations shorten the paths in `crate::user`. Since the harness is in the same crate,
`pub(crate)` also works for `Item::price` and `pricing` (`pub(crate) mod pricing`).

```rust
// `shop` as in the solution.

use shop::cart::Cart;
use shop::pricing::{discounted, with_tax};

pub fn sample_cart() -> Cart {
    let mut cart = Cart::new();
    for (name, price) in [("pen", 120), ("book", 1500)] {
        cart.add(shop::Item::new(name, price));
    }
    cart
}

pub fn checkout(cart: &Cart, percent: u32) -> u32 {
    with_tax(discounted(cart.total(), percent))
}
```
