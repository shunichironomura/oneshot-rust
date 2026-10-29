use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    items.iter().map(|item| item.parse::<i32>()).collect()
}
