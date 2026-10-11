use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct BadField {
    pub column: usize,
    pub text: String,
}

/// Parses column `i` of a row after trimming it. A missing column is read as "".
fn field<T: FromStr>(cols: &[&str], i: usize) -> Result<T, BadField> {
    let text = cols.get(i).copied().unwrap_or("").trim();
    text.parse().map_err(|_| BadField { column: i, text: text.to_string() })
}

/// Checks that the id, age and score of a row can be read. The values themselves are not needed.
pub fn validate_row(line: &str) -> Result<(), BadField> {
    let cols: Vec<&str> = line.split(',').collect();
    field(&cols, 0)?; // id
    field(&cols, 1)?; // age
    field(&cols, 2)?; // score
    Ok(())
}

pub fn sum_ids(lines: &[&str]) -> Result<u64, BadField> {
    let mut total = 0;
    for line in lines {
        validate_row(line)?;
        let cols: Vec<&str> = line.split(',').collect();
        let id: u32 = field(&cols, 0)?;
        total += u64::from(id);
    }
    Ok(total)
}
