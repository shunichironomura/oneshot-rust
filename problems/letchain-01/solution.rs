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
    if let Ok(n) = tok.parse::<u64>() {
        return match n {
            0..10 => Token::Tiny(n),
            10..1000 => Token::Small(n),
            1000.. => Token::Large(n),
        };
    }
    // Look for the closing quote only after the opening one: a lone `'` is both the first and the
    // last character of the token, but it is not a pair of quotes.
    if let Some(rest) = tok.strip_prefix('\'')
        && let Some(text) = rest.strip_suffix('\'')
        && !text.contains('\'')
    {
        return Token::Quoted(text.to_string());
    }
    if let Some((before, after)) = tok.split_once(':')
        && !before.is_empty()
        && !after.is_empty()
    {
        return Token::Pair(before.to_string(), after.to_string());
    }
    Token::Word(tok.to_string())
}

pub fn sum_first_two(nums: &[u8]) -> Option<u8> {
    match nums {
        [a, b, ..] => a.checked_add(*b),
        _ => None,
    }
}
