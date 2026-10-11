#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(u64),
    Ident(String),
    Op(char),
    Arrow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    /// A character that cannot start a token, and its position.
    Unexpected { ch: char, pos: usize },
    /// A number greater than `u64::MAX`, and the position of its first digit.
    Overflow { pos: usize },
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    todo!()
}
