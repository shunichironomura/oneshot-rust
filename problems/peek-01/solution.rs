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
    // `enumerate` on `chars` counts characters, not bytes.
    let mut chars = src.chars().enumerate().peekable();
    let mut tokens = Vec::new();
    while let Some((pos, c)) = chars.next() {
        if c.is_whitespace() {
            continue;
        }
        let token = match c {
            '0'..='9' => {
                let mut digits = String::from(c);
                // `next_if` consumes the next character only if it is a digit.
                while let Some((_, d)) = chars.next_if(|&(_, d)| d.is_ascii_digit()) {
                    digits.push(d);
                }
                // Only ASCII digits, and the statement says the value fits in a `u64`.
                Token::Num(digits.parse().expect("a run of digits that fits in a u64"))
            }
            '-' => match chars.next_if(|&(_, d)| d == '>') {
                Some(_) => Token::Arrow,
                None => Token::Op('-'),
            },
            '+' | '*' | '/' | '(' | ')' | '=' => Token::Op(c),
            _ => return Err(LexError::Unexpected { ch: c, pos }),
        };
        tokens.push(token);
    }
    Ok(tokens)
}
