#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
    /// Flashes yellow; the number is how many more flashes remain after this one.
    Flashing(u32),
}

impl Light {
    pub fn next(self) -> Light {
        todo!()
    }

    pub fn duration_secs(self) -> u32 {
        todo!()
    }

    pub fn time_until_green(self) -> u32 {
        todo!()
    }
}
