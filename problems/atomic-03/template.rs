pub struct Stats {
    count: u64,
    sum: u64,
    max: Option<u64>,
    min: Option<u64>,
}

impl Stats {
    pub const fn new() -> Stats {
        Stats { count: 0, sum: 0, max: None, min: None }
    }

    pub fn record(&self, value: u64) {
        todo!()
    }

    pub fn count(&self) -> u64 {
        todo!()
    }

    pub fn sum(&self) -> u64 {
        todo!()
    }

    pub fn max(&self) -> Option<u64> {
        todo!()
    }

    pub fn min(&self) -> Option<u64> {
        todo!()
    }
}

pub static GLOBAL: Stats = Stats::new();
