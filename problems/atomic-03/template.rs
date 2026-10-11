pub struct Stats {
    count: u64,
    sum: f64,
    max: Option<f64>,
    min: Option<f64>,
}

impl Stats {
    pub const fn new() -> Stats {
        Stats { count: 0, sum: 0.0, max: None, min: None }
    }

    pub fn record(&self, value: f64) {
        todo!()
    }

    pub fn count(&self) -> u64 {
        todo!()
    }

    pub fn sum(&self) -> f64 {
        todo!()
    }

    pub fn max(&self) -> Option<f64> {
        todo!()
    }

    pub fn min(&self) -> Option<f64> {
        todo!()
    }
}

pub static GLOBAL: Stats = Stats::new();
