#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(u64),
    Op(char),
    Arrow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    /// A character that cannot start a token, and its position.
    Unexpected { ch: char, pos: usize },
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    todo!()
}
