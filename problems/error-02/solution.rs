use std::error::Error;

pub fn total(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut sum: i64 = 0;
    let mut seen = false;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let n: i64 = line.parse()?;
        sum = sum.checked_add(n).ok_or("overflow")?;
        seen = true;
    }
    if !seen {
        return Err("no numbers".into());
    }
    Ok(sum)
}
