use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    items.iter().map(|item| item.parse::<i32>()).collect()
}

pub fn split_parsed<'a>(items: &[&'a str]) -> (Vec<i32>, Vec<&'a str>) {
    let mut values = Vec::new();
    let mut invalid = Vec::new();
    for &item in items {
        match item.parse::<i32>() {
            Ok(n) => values.push(n),
            Err(_) => invalid.push(item),
        }
    }
    (values, invalid)
}

pub fn double_first(items: &[&str]) -> Result<Option<i64>, ParseIntError> {
    let first = items.first().map(|item| item.parse::<i32>()).transpose()?;
    Ok(first.map(|n| 2 * i64::from(n)))
}

pub fn sum_all(xs: &[Option<u32>]) -> Option<u32> {
    xs.iter().try_fold(0u32, |total, &x| total.checked_add(x?))
}
