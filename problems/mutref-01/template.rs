use std::collections::HashMap;

pub struct Labels {
    cache: HashMap<u32, String>,
    built: usize,
}

impl Labels {
    pub fn new() -> Labels {
        Labels {
            cache: HashMap::new(),
            built: 0,
        }
    }

    pub fn get(&mut self, id: u32) -> &str {
        self.cache.entry(id).or_insert_with(|| self.build(id))
    }

    pub fn built(&self) -> usize {
        self.built
    }

    fn build(&mut self, id: u32) -> String {
        self.built += 1;
        format!("item-{id}")
    }
}
