#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

pub fn is_freezing(t: impl Into<Celsius>) -> bool {
    let c: Celsius = t.into();
    c.0 <= 0.0
}

pub fn average<T: Into<Celsius>>(readings: Vec<T>) -> Option<Celsius> {
    if readings.is_empty() {
        return None;
    }
    let count = readings.len() as f64;
    let total: f64 = readings
        .into_iter()
        .map(|r| {
            let c: Celsius = r.into();
            c.0
        })
        .sum();
    Some(Celsius(total / count))
}

pub fn warmest(readings: &[Fahrenheit]) -> Option<Celsius> {
    readings
        .iter()
        .map(|&f| Celsius::from(f))
        .max_by(|a, b| a.0.total_cmp(&b.0))
}
