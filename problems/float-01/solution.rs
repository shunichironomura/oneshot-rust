fn valid(v: &[f64]) -> impl Iterator<Item = f64> {
    v.iter().copied().filter(|x| !x.is_nan())
}

pub fn max_reading(v: &[f64]) -> Option<f64> {
    valid(v).max_by(f64::total_cmp)
}

fn min_reading(v: &[f64]) -> Option<f64> {
    valid(v).min_by(f64::total_cmp)
}

pub fn sorted_readings(v: &[f64]) -> Vec<f64> {
    let mut readings: Vec<f64> = valid(v).collect();
    readings.sort_by(f64::total_cmp);
    readings
}

pub fn spread(v: &[f64]) -> Option<f64> {
    Some(max_reading(v)? - min_reading(v)?)
}
