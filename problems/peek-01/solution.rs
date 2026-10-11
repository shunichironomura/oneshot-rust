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

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
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
                // Only digits, at least one: the only possible error is overflow.
                let value = digits.parse::<u64>().map_err(|_| LexError::Overflow { pos })?;
                Token::Num(value)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut name = String::from(c);
                while let Some((_, d)) = chars.next_if(|&(_, d)| is_ident_char(d)) {
                    name.push(d);
                }
                Token::Ident(name)
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
