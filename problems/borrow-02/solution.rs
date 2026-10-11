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

    pub fn checkout(&mut self) {
        self.total = 0;
        self.biggest = None;
        for item in &self.items {
            let subtotal = item.price * item.qty;
            self.total += subtotal;
            let is_bigger = match &self.biggest {
                Some((_, best)) => subtotal > *best,
                None => true,
            };
            if is_bigger {
                self.biggest = Some((item.name.clone(), subtotal));
            }
        }
    }

    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn biggest(&self) -> Option<&str> {
        self.biggest.as_ref().map(|(name, _)| name.as_str())
    }
}
