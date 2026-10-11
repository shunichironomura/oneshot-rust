#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        todo!()
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        todo!()
    }
}

pub fn is_freezing(t: impl Into<Celsius>) -> bool {
    todo!()
}

pub fn average<T: Into<Celsius>>(readings: Vec<T>) -> Option<Celsius> {
    todo!()
}

pub fn warmest(readings: &[Fahrenheit]) -> Option<Celsius> {
    todo!()
}
