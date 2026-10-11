#[derive(Debug, PartialEq)]
pub enum Token {
    Tiny(u64),
    Small(u64),
    Large(u64),
    Quoted(String),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token {
    todo!()
}

pub fn sum_first_two(nums: &[u8]) -> Option<u8> {
    todo!()
}
