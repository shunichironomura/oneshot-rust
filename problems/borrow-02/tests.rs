use crate::judge::{case, check_eq};
use crate::user::{Cart, Item};

fn item(name: &str, price: u32, qty: u32) -> Item {
    Item { name: name.to_string(), price, qty }
}

pub fn run() {
    case("example-1", || {
        let mut cart = Cart::new(vec![item("apple", 120, 3), item("melon", 500, 1), item("tea", 150, 2)]);
        cart.checkout();
        check_eq((cart.total(), cart.biggest()), (1160, Some("melon")))
    });
    case("case-1", || {
        let mut cart = Cart::new(Vec::new());
        cart.checkout();
        check_eq((cart.total(), cart.biggest()), (0, None))
    });
    case("case-2", || {
        // On a tie, the earlier item stays the biggest.
        let mut cart = Cart::new(vec![item("a", 100, 2), item("b", 200, 1), item("c", 50, 1)]);
        cart.checkout();
        check_eq((cart.total(), cart.biggest()), (450, Some("a")))
    });
    case("case-3", || {
        // Checking out twice gives the same result, not a doubled total.
        let mut cart = Cart::new(vec![item("x", 7, 3), item("y", 1, 30)]);
        cart.checkout();
        cart.checkout();
        check_eq((cart.total(), cart.biggest()), (51, Some("y")))
    });
    case("case-4", || {
        // A quantity of 0 gives a subtotal of 0, which is still the biggest of one item.
        let mut cart = Cart::new(vec![item("ghost", 999, 0)]);
        cart.checkout();
        check_eq((cart.total(), cart.biggest()), (0, Some("ghost")))
    });
}
