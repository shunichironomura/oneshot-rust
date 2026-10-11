pub struct Item {
    pub name: String,
    pub price: u32,
    pub qty: u32,
}

pub struct Cart {
    items: Vec<Item>,
    total: u32,
    biggest: Option<(String, u32)>,
}

impl Cart {
    pub fn new(items: Vec<Item>) -> Self {
        Cart { items, total: 0, biggest: None }
    }

    fn items(&self) -> &[Item] {
        &self.items
    }

    fn record(&mut self, name: &str, subtotal: u32) {
        self.total += subtotal;
        let is_bigger = match &self.biggest {
            Some((_, best)) => subtotal > *best,
            None => true,
        };
        if is_bigger {
            self.biggest = Some((name.to_string(), subtotal));
        }
    }

    pub fn checkout(&mut self) {
        self.total = 0;
        self.biggest = None;
        for item in self.items() {
            self.record(&item.name, item.price * item.qty);
        }
    }

    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn biggest(&self) -> Option<&str> {
        self.biggest.as_ref().map(|(name, _)| name.as_str())
    }
}
