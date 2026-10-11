#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Int(i64),
    Float(f64),
    Text(String),
    Empty,
}

pub fn parse_row(line: &str) -> Vec<Cell> {
    line.split(',').map(|field| parse_cell(field.trim())).collect()
}

fn parse_cell(field: &str) -> Cell {
    if field.is_empty() {
        Cell::Empty
    } else if let Ok(n) = field.parse::<i64>() {
        Cell::Int(n)
    } else if let Ok(x) = field.parse::<f64>() {
        Cell::Float(x)
    } else {
        Cell::Text(field.to_string())
    }
}

pub fn numeric_sum(row: &[Cell]) -> f64 {
    let mut sum = 0.0;
    for cell in row {
        // `cell` is a `&Cell`, so `n` is a `&i64` and `x` is a `&f64`.
        match cell {
            Cell::Int(n) => sum += *n as f64,
            Cell::Float(x) => sum += *x,
            Cell::Text(_) | Cell::Empty => {}
        }
    }
    sum
}

pub fn texts(row: &[Cell]) -> Vec<&str> {
    row.iter()
        .filter_map(|cell| match cell {
            Cell::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect()
}
