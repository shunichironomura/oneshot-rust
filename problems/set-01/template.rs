#[derive(Debug, PartialEq)]
pub struct Overlap {
    pub both: Vec<String>,
    pub only_left: Vec<String>,
    pub only_right: Vec<String>,
    pub exactly_one: Vec<String>,
    pub all: Vec<String>,
}

pub fn overlap(left: &[&str], right: &[&str]) -> Overlap {
    todo!()
}
