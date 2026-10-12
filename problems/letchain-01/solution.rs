#[derive(Debug, PartialEq)]
pub enum Token {
    Quoted(String),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token {
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
