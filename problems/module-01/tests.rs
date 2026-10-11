use crate::judge::{case, check_eq};
use crate::user::shop::cart::Cart;
use crate::user::shop::{Item, TAX_RATE};
use crate::user::{checkout, sample_cart};

fn cart_of(items: &[(&str, u32)]) -> Cart {
    let mut cart = Cart::new();
    for &(name, price) in items {
        cart.add(Item::new(name, price));
    }
    cart
}

pub fn run() {
    case("sample-1", || {
        let cart = sample_cart();
        check_eq(cart.total(), 1620)?;
        check_eq(cart.len(), 2)?;
        check_eq(TAX_RATE, 10)
    });
    case("sample-2", || check_eq(checkout(&sample_cart(), 10), 1603));
    case("hidden-1", || {
        // An empty cart totals 0, whatever the discount.
        let cart = Cart::new();
        check_eq(cart.total(), 0)?;
        check_eq(cart.len(), 0)?;
        check_eq(checkout(&cart, 0), 0)?;
        check_eq(checkout(&cart, 50), 0)
    });
    case("hidden-2", || {
        let tea = Item::new("tea", 300);
        check_eq(tea.name(), "tea")?;
        check_eq(tea.price(), 300)?;
        let free = Item::new("お茶", 0);
        check_eq(free.name(), "お茶")?;
        check_eq(free.price(), 0)
    });
    case("hidden-3", || {
        // A cart built by the harness; the same item may be added twice.
        let cart = cart_of(&[("tea", 300), ("cake", 450), ("tea", 300)]);
        check_eq(cart.len(), 3)?;
        check_eq(cart.total(), 1050)?;
        check_eq(checkout(&cart, 20), 924)
    });
    case("hidden-4", || {
        // No discount, a full discount and a quarter off.
        let cart = sample_cart();
        check_eq(checkout(&cart, 0), 1782)?;
        check_eq(checkout(&cart, 100), 0)?;
        check_eq(checkout(&cart, 25), 1336)
    });
    case("hidden-5", || {
        // Both the discount and the tax are rounded down, and the discount comes first.
        check_eq(checkout(&cart_of(&[("vase", 999)]), 10), 990)?;
        check_eq(checkout(&cart_of(&[("clip", 19)]), 50), 11)?;
        check_eq(checkout(&cart_of(&[("gum", 7)]), 0), 7)
    });
    case("hidden-6", || {
        // Every call returns a new cart, and it can be extended.
        let mut cart = sample_cart();
        cart.add(Item::new("ink", 80));
        check_eq(cart.len(), 3)?;
        check_eq(cart.total(), 1700)?;
        check_eq(sample_cart().len(), 2)
    });
}
