use std::num::ParseIntError;

pub fn sum_csv(s: &str) -> Result<i64, ParseIntError> {
    if s.trim().is_empty() {
        return Ok(0);
    }
    let mut total = 0;
    for item in s.split(',') {
        total += item.trim().parse::<i64>()?;
    }
    Ok(total)
}
