pub fn moving_average(values: &[i32], window: usize) -> Vec<f64> {
    let mut averages = Vec::new();
    if window == 0 || window > values.len() {
        return averages;
    }
    let sum: i64 = 0;
    for i in 0..values.len() {
        sum += values[i];
        if i >= window {
            sum -= values[i - window];
        }
        if i + 1 >= window {
            averages.push(sum / window);
        }
    }
    averages
}
