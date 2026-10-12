#[derive(Debug, PartialEq)]
pub enum Token {
    Quoted(String),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token {
    todo!()
}
