#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Int(i64),
    Text(String),
    Empty,
}

pub fn parse_row(line: &str) -> Vec<Cell> {
    todo!()
}

pub fn numeric_sum(row: &[Cell]) -> i64 {
    todo!()
}

pub fn texts(row: &[Cell]) -> Vec<&str> {
    todo!()
}
