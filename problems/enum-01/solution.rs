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
        match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
            Light::Flashing(0) => Light::Red,
            Light::Flashing(n) => Light::Flashing(n - 1),
        }
    }

    pub fn duration_secs(self) -> u32 {
        match self {
            Light::Red => 30,
            Light::Yellow => 5,
            Light::Green => 25,
            Light::Flashing(_) => 1,
        }
    }

    pub fn time_until_green(self) -> u32 {
        let mut light = self;
        let mut total = 0;
        while light != Light::Green {
            total += light.duration_secs();
            light = light.next();
        }
        total
    }
}
