pub mod shop {
    pub const TAX_RATE: u32 = 10;

    pub struct Item {
        name: String,
        price: u32,
    }

    impl Item {
        pub fn new(name: &str, price: u32) -> Item {
            Item { name: name.to_string(), price }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn price(&self) -> u32 {
            self.price
        }
    }

    pub mod cart {
        pub struct Cart {
            items: Vec<super::Item>,
        }

        impl Cart {
            pub fn new() -> Cart {
                Cart { items: Vec::new() }
            }

            pub fn add(&mut self, item: super::Item) {
                self.items.push(item);
            }

            pub fn total(&self) -> u32 {
                // `price` is private to `shop`, and `cart` is inside `shop`, so this is allowed.
                self.items.iter().map(|item| item.price).sum()
            }

            pub fn len(&self) -> usize {
                self.items.len()
            }
        }
    }

    // Visible in the parent of `shop` (where `checkout` is), and no further.
    pub(super) mod pricing {
        use super::TAX_RATE;

        pub fn discounted(total: u32, percent: u32) -> u32 {
            total - total * percent / 100
        }

        pub fn with_tax(total: u32) -> u32 {
            total + total * TAX_RATE / 100
        }
    }
}

pub fn sample_cart() -> shop::cart::Cart {
    let mut cart = shop::cart::Cart::new();
    cart.add(shop::Item::new("pen", 120));
    cart.add(shop::Item::new("book", 1500));
    cart
}

pub fn checkout(cart: &shop::cart::Cart, percent: u32) -> u32 {
    shop::pricing::with_tax(shop::pricing::discounted(cart.total(), percent))
}
